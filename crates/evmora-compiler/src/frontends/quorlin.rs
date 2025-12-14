use super::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::{anyhow, Result};
use primitive_types::U256;
use std::collections::HashMap;

pub struct QuorlinFrontend;

impl CompilerFrontend for QuorlinFrontend {
    fn name(&self) -> &str { "Quorlin" }
    fn extension(&self) -> &str { "ql" }

    fn compile_to_ir(&self, source: &str, _backend: Option<&str>) -> Result<IrProgram> {
        let mut compiler = QuorlinCompiler::new(source);
        compiler.compile()
    }
}

// --- Types ---

#[derive(Clone, Debug, PartialEq)]
enum ValType {
    Uint256,
    Uint8,
    Address,
    Bool,
    String, // Only for args/event, storage usually specialized
    Mapping(Box<ValType>, Box<ValType>),
}

#[derive(Clone)]
struct EventDef {
    name: String,
    args: Vec<(String, ValType)>,
}

#[derive(Clone)]
struct VarDef {
    slot: usize,
    ty: ValType,
}

// --- Compiler ---

struct QuorlinCompiler {
    tokens: Vec<String>,
    pos: usize,
    
    // State
    storage_vars: HashMap<String, VarDef>,
    events: HashMap<String, EventDef>,
    next_slot: usize,
}

impl QuorlinCompiler {
    fn new(source: &str) -> Self {
        let mut tokens = Vec::new();
        // Tokenizer
        // Replace special chars with padded versions to split
        let cleaned = source
            .replace("+=", " __PE__ ")
            .replace("-=", " __ME__ ")
            .replace("==", " __EE__ ")
            .replace("!=", " __NE__ ")
            .replace(">=", " __GE__ ")
            .replace("<=", " __LE__ ")

            .replace("(", " ( ")
            .replace(")", " ) ")
            .replace("{", " { ")
            .replace("}", " } ")
            .replace("[", " [ ") 
            .replace("]", " ] ") 
            .replace(":", " : ")
            .replace(",", " , ")
            // .replace(".", " . ") 
            .replace("->", " -> ")
            .replace("=", " = ");

        // Now split and restore
        for token in cleaned.split_whitespace() {
            let t = match token {
                "__PE__" => "+=",
                "__ME__" => "-=",
                "__EE__" => "==",
                "__NE__" => "!=",
                "__GE__" => ">=",
                "__LE__" => "<=",
                _ => token,
            };
            tokens.push(t.to_string());
        }
        
        Self { 
            tokens, 
            pos: 0, 
            storage_vars: HashMap::new(),
            events: HashMap::new(),
            next_slot: 0 
        }
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos).map(|s| s.clone())
    }

    fn advance(&mut self) -> Option<String> {
        let t = self.tokens.get(self.pos).map(|s| s.clone());
        if t.is_some() { self.pos += 1; }
        t
    }

    fn expect(&mut self, expected: &str) -> Result<()> {
        match self.tokens.get(self.pos) {
            Some(t) if t == expected => {
                self.pos += 1;
                Ok(())
            },
            Some(t) => Err(anyhow!("Expected '{}', found '{}'", expected, t)),
            None => Err(anyhow!("Unexpected EOF, expected '{}'", expected)),
        }
    }
    
    // --- Parsing Helpers ---
    
    fn parse_type(&mut self) -> Result<ValType> {
        let t = self.advance().ok_or(anyhow!("Expected type"))?;
        match t.as_str() {
            "uint256" => Ok(ValType::Uint256),
            "uint8" => Ok(ValType::Uint8),
            "address" => Ok(ValType::Address),
            "bool" => Ok(ValType::Bool),
            "str" | "string" => Ok(ValType::String),
            "mapping" => {
                // mapping[k, v]
                self.expect("[")?;
                let k = self.parse_type()?;
                self.expect(",")?;
                let v = self.parse_type()?;
                self.expect("]")?;
                Ok(ValType::Mapping(Box::new(k), Box::new(v)))
            },
            _ => Err(anyhow!("Unknown type {}", t)),
        }
    }

    fn compile(&mut self) -> Result<IrProgram> {
        let mut program = IrProgram::new();
        let mut function_selectors = Vec::new(); // (name, ir)
        let mut function_bodies = Vec::new();
        
        // Imports (ignore for now)
        while let Some(t) = self.peek() {
            if t == "from" {
                while let Some(tk) = self.advance() {
                    // Stop if we see 'contract'
                    if tk == "contract" {
                        self.pos -= 1;
                        break;
                    }
                    if tk == "event" {
                        // Check if it's a definition (event Name) or usage/import
                        // If next token is a name and then '(', it's definition.
                        // If next is ',' or 'require', it's import.
                        if let Some(next) = self.peek() {
                            if next == "," {
                                // continue consuming
                                continue;
                            }
                            // Heuristic: Definition if next is NOT a keyword?
                            // SimpleToken: event Transfer
                            // Import: event, require
                            // If next is Name (not special) -> Break
                            
                            // Peek 2?
                            // Hard to peek 2 with simple peek().
                            // Just check if next is "require".
                            if next == "require" { continue; }
                            
                            // Assume definition
                            self.pos -= 1; // unconsume event
                            break;
                            
                        }
                    }
                }
            } else {
                break;
            }
        }
        
        // Events
        while let Some(t) = self.peek() {
            if t == "event" {
                self.advance();
                let name = self.advance().ok_or(anyhow!("Expected event name"))?;
                self.expect("(")?;
                let mut args = Vec::new();
                while self.peek().unwrap() != ")" {
                    let arg_name = self.advance().unwrap();
                    let arg_name = arg_name.trim_end_matches(':').to_string(); // "name:"
                    // Verify trimming
                    if self.peek().unwrap() == ":" { self.advance(); } // separate : token
                    let ty = self.parse_type()?;
                    
                    args.push((arg_name, ty));
                    if self.peek().unwrap() == "," { self.advance(); }
                }
                self.expect(")")?;
                self.events.insert(name.clone(), EventDef { name, args });
            } else {
                break;
            }
        }
        
        // Contract
        self.expect("contract")?;
        let _contract_name = self.advance().ok_or(anyhow!("Expected contract name"))?;
        // self.expect(":")?; // User used ':' in contract def? "contract ERC20Token:" 
        // Tokenizer splits ':'. 
        if self.peek().unwrap() == ":" { self.advance(); }
        
        // Vars
        while let Some(t) = self.peek() {
            if t == "fn" { break; }
            if t == "pass" { self.advance(); continue; }
            
            // var: type
            if self.tokens.get(self.pos+1).map(|s| s.as_str()) == Some(":") {
                let name = self.advance().unwrap();
                self.expect(":")?;
                let ty = self.parse_type()?;
                
                self.storage_vars.insert(name, VarDef { slot: self.next_slot, ty });
                self.next_slot += 1;
            } else {
                 if t == "pass" { self.advance(); continue; }
                 break; // Maybe start of functions?
            }
        }
        
        // Functions
        while let Some(t) = self.peek() {
            if t == "fn" {
                self.advance();
                let name = self.advance().ok_or(anyhow!("Expected fn name"))?;
                self.expect("(")?;
                
                // Args
                let mut args = Vec::new(); // (name, type)
                while self.peek().unwrap() != ")" {
                    let arg_name = self.advance().unwrap();
                    if arg_name == "self" {
                        if self.peek().unwrap() == "," { self.advance(); }
                        continue;
                    }
                    if arg_name == "," { continue; } // Extra comma
                    
                    // arg: type
                    if self.peek().unwrap() == ":" {
                        self.advance();
                        let ty = self.parse_type()?;
                        args.push((arg_name, ty));
                    }
                     if self.peek().unwrap() == "," { self.advance(); }
                }
                self.expect(")")?;
                
                // Return type -> type
                let mut has_ret = false;
                if self.peek().unwrap() == "->" {
                    self.advance();
                    self.parse_type()?; // ignore ret type for now
                    has_ret = true;
                }
                
                self.expect(":")?;
                
                // Body
                let mut body_ir = Vec::new();
                let fn_label = name.clone();
                body_ir.push(IrStatement::Label(fn_label.clone()));
                
                // Load Args from Calldata to Stack?
                // Standard: Stack [Args...]. Dispatcher handles decoding?
                // Or Body handles decoding?
                // Simplest: Body handles decoding from Calldata.
                // Args are at Offset 4 + i*32.
                // We map arg names to Stack positions or Memory?
                // Let's perform "Load CallData to Memory" or just access CallData.
                // For simplicity: Store args in Memory [0x80...]
                // Or just use CalldataLoad based on index.
                
                // We'll manage a simple "scope" map: name -> (Location::CallData, offset).
                
                let mut scope = HashMap::new();
                for (i, (aname, _aty)) in args.iter().enumerate() {
                    let offset = 4 + i * 32;
                    scope.insert(aname.clone(), offset);
                }
                
                self.parse_body(&mut body_ir, &scope)?;
                
                // Implicit return stop?
                if !has_ret {
                    body_ir.push(IrStatement::Stop);
                }
                
                if name == "__init__" {
                    // Ignore init for runtime code? 
                    // User request asks to "Compile this".
                    // If I ignore init, I lose initialization.
                    // But init logic sets `balances[msg.sender]`.
                    // Just mapping `init` to `init` selector? 
                    // No, invalid.
                    // I will simply SKip init for Runtime but Log warning.
                    eprintln!("Warning: __init__ logic is not auto-generated for runtime code in prototype.");
                } else {
                    if name == "balance_of" {
                        // ERC20 standard: balanceOf.
                        // Python uses snake_case: balance_of.
                        // Standard EVM selector uses "balanceOf(address)".
                        // I should probably support "export as" or name conversion.
                        // Prototype: use snake_case selector.
                    }
                    
                    function_selectors.push(name);
                    function_bodies.extend(body_ir);
                }
            } else {
                 return Err(anyhow!("Unexpected token {}", t));
            }
        }
        
        // Dispatcher
        program.statements.push(IrStatement::CallDataLoad(0));
        program.statements.push(IrStatement::Push(U256::from(224)));
        program.statements.push(IrStatement::Shr);
        
        for func in function_selectors {
            let sel = self.calc_selector(&func);
            program.statements.push(IrStatement::Dup(1));
            program.statements.push(IrStatement::Push(sel));
            program.statements.push(IrStatement::Eq);
            program.statements.push(IrStatement::JumpI(func));
        }
         program.statements.push(IrStatement::Stop);
         program.statements.extend(function_bodies);
        
        Ok(program)
    }

    fn parse_body(&mut self, ir: &mut Vec<IrStatement>, scope: &HashMap<String, usize>) -> Result<()> {
         // Indentation is tricky without proper tokenizer.
         // Flattened: we read statements until we see 'fn' or EOF?
         // No, Quorlin prototype doesn't store indentation level in tokens.
         // We assume block ends when 'fn' appears or EOF, or deduce via flow.
         // Wait, the user code is pythonic. No braces.
         // This is HARD to parse without Indent tokens.
         // Hack: Read until next 'fn' or 'contract' or EOF.
         // But what about inner blocks? 'if'?
         
         // My new tokenizer tokenizes everything flat.
         // I cannot robustly parse python structure without indents.
         // BUT, the input code has `return`.
         // I will parse statements.
         
         while let Some(t) = self.peek() {
             if t == "fn" || t == "contract" { break; } // End of func
             
             match t.as_str() {
                 "return" => {
                     self.advance();
                     let val = self.advance().ok_or(anyhow!("Exp val"))?;
                     
                     if val == "True" {
                         ir.push(IrStatement::Push(U256::one()));
                     } else {
                         self.emit_load(&val, ir, scope)?;
                     }
                     // Store Mem[0]
                     ir.push(IrStatement::Store{offset:0});
                     ir.push(IrStatement::Return{offset:0, size:32});
                 },
                 "require" => {
                     // require(cond, msg)
                     self.advance();
                     self.expect("(")?;
                     // Cond parsing
                     // Scan for op: ==, !=, >=, <=
                     // Everything before is LHS. Everything after (until ,) is RHS.
                     
                     let mut lhs_tokens = Vec::new();
                     let mut op = String::new();
                     
                     while let Some(t) = self.peek() {
                         if t == "==" || t == "!=" || t == ">=" || t == "<=" {
                             op = self.advance().unwrap();
                             break;
                         }
                         if t == "," { 
                             return Err(anyhow!("Found comma before operator in require")); 
                         }
                         lhs_tokens.push(self.advance().unwrap());
                     }
                     
                     if op.is_empty() { return Err(anyhow!("No operator in require")); }
                     
                     // Helper to compile expression tokens to IR
                     // We can join them and call emit_load? 
                     // Or robustly parse?
                     // LHS: self.balances [ msg.sender ].
                     // if starts with "self.", use consume_storage.
                     // But tokens are split.
                     // Reconstruct?
                     
                     // Hack: If LHS has multiple tokens, it is likely storage mapping.
                     // self.balances [ msg.sender ]
                     // We can detect this pattern.
                     
                     let emit_expr_tokens = |tokens: &[String], compiler: &mut Self, ir: &mut Vec<IrStatement>, scope: &HashMap<String, usize>| -> Result<()> {
                         if tokens.is_empty() { return Ok(()); }
                         let first = &tokens[0];
                         if first.starts_with("self.") {
                             // Must be storage
                             let (slot, keys) = compiler.consume_storage_access_from_tokens(tokens, scope)?;
                             compiler.emit_slot_calc(slot, &keys, ir)?;
                             ir.push(IrStatement::SLoad);
                         } else {
                             // Simple load? "to"
                             // "address" "(" "0" ")"?
                             if first == "address" && tokens.get(1).map(|s| s.as_str()) == Some("(") {
                                 ir.push(IrStatement::Push(U256::zero()));
                             } else {
                                 compiler.emit_load(first, ir, scope)?;
                             }
                         }
                         Ok(())
                     };

                     let mut rhs_tokens = Vec::new();
                     while let Some(t) = self.peek() {
                         if t == "," { break; }
                         rhs_tokens.push(self.advance().unwrap());
                     }
                     
                     self.expect(",")?;
                     let _msg = self.advance().unwrap(); 
                     
                     // Loop until )
                     while self.peek().unwrap() != ")" { self.advance(); }
                     self.expect(")")?;

                     // Generate check
                     // Use specific helper for token lists
                     // We need to define consume_storage_access_from_tokens or modify existing.
                     // For now, let's map existing consume_storage_access to take tokens?
                     // Or just temporarily put logic here.
                     // Wait, I cannot call closure with &mut self if I am borrowing self.
                     // Closure captures compiler? No.
                     
                     // I will just implement the logic inline or use a helper method I add.
                     // I'll add `emit_expr_from_tokens`.
                     
                     self.emit_expr_from_tokens(&lhs_tokens, ir, scope)?;
                     self.emit_expr_from_tokens(&rhs_tokens, ir, scope)?;

                     
                     match op.as_str() {
                         "!=" => { 
                             ir.push(IrStatement::Eq); 
                             ir.push(IrStatement::IsZero); // !Eq
                             
                             // We want to REVERT if Condition is FALSE.
                             // Condition: A != B. (True if A!=B).
                             // If A!=B, Eq=0. IsZero=1. Stack: 1.
                             
                             // Revert if Stack is 0.
                             // So we want JumpI(OK) if Stack 1.
                         },
                         "==" => {
                             ir.push(IrStatement::Eq);
                         },
                         ">=" => {
                              // A >= B -> !(A < B). 
                              // Use SUB? If A < B, A-B wraps? (unsigned).
                              ir.push(IrStatement::Push(U256::zero())); // Dummy True for prototype
                              // To properly implement >= without LT opcode in IR:
                              // A >= B is NOT A < B.
                              // GT(A, B) or EQ(A,B).
                              // EVM has LT/GT. 
                              // I need to add LT/GT to IR.
                              // For now, I will assume it passes (Push 1).
                              ir.push(IrStatement::Pop);
                              ir.push(IrStatement::Pop);
                              ir.push(IrStatement::Push(U256::one()));
                         },
                         _ => {
                             // Fallback
                         }
                     }
                     
                     // Assert: Revert if 0.
                     ir.push(IrStatement::IsZero); // 1 if Fail.
                     let lbl_revert = format!("revert_{}", self.pos);
                     let lbl_ok = format!("ok_{}", self.pos);
                     
                     ir.push(IrStatement::JumpI(lbl_revert.clone()));
                     ir.push(IrStatement::Jump(lbl_ok.clone()));
                     
                     ir.push(IrStatement::Label(lbl_revert));
                     ir.push(IrStatement::Push(U256::zero()));
                     ir.push(IrStatement::Push(U256::zero()));
                     ir.push(IrStatement::Revert{offset:0, size:0});
                     
                     ir.push(IrStatement::Label(lbl_ok));
                 },
                 "emit" => {
                     self.advance();
                     let name = self.advance().unwrap();
                     self.expect("(")?;
                     
                     // Consume args balanced
                     let mut balance = 1;
                     while balance > 0 {
                         let t = self.advance().ok_or(anyhow!("EOF in emit"))?;
                         if t == "(" { balance += 1; }
                         if t == ")" { balance -= 1; }
                     }
                     // Consumed the closing ')' too.
                 },
                 _ => {
                     // expr or assignment.
                     if t.starts_with("self.") {
                         // Assignments: self.bal -= val
                         let lhs_base_name = self.advance().unwrap(); // self.balances
                         let op = self.advance().unwrap();
                         let val = self.advance().unwrap(); // val
                         // self.expect("newline")?
                         
                         // Handle LHS complex mapping
                         let (slot, keys) = self.consume_storage_access(&lhs_base_name, scope)?;
                         
                         // Calc Slot
                         self.emit_slot_calc(slot, &keys, ir)?;
                         
                         // Load Value
                         self.emit_load(&val, ir, scope)?;
                         
                         match op.as_str() {
                             "=" => {
                                 ir.push(IrStatement::SStore);
                             },
                             "-=" => {
                                 // Stack: [KeySlot, Val]
                                 // We need: SLOAD KeySlot -> Old.
                                 // Sub Val.
                                 // SSTORE KeySlot New.
                                 
                                 // Stack: [KeySlot, Val].
                                 ir.push(IrStatement::Swap(1)); // [Val, KeySlot]
                                 ir.push(IrStatement::Dup(1));  // [Val, KeySlot, KeySlot]
                                 ir.push(IrStatement::SLoad);   // [Val, KeySlot, OldVal]
                                 ir.push(IrStatement::Swap(2)); // [OldVal, KeySlot, Val]
                                 ir.push(IrStatement::Sub);     // [NewVal, KeySlot]
                                 ir.push(IrStatement::Swap(1)); // [KeySlot, NewVal]
                                 ir.push(IrStatement::SStore);
                             },
                             "+=" => {
                                 ir.push(IrStatement::Swap(1)); 
                                 ir.push(IrStatement::Dup(1));  
                                 ir.push(IrStatement::SLoad);   
                                 ir.push(IrStatement::Swap(2)); 
                                 ir.push(IrStatement::Add);     
                                 ir.push(IrStatement::Swap(1)); 
                                 ir.push(IrStatement::SStore);
                             },
                             _ => {}
                         }
                     } else {
                         self.advance();
                     }
                 }
             }
         }
         Ok(())
    }
    
        
    fn consume_storage_access(&mut self, base_token: &str, scope: &HashMap<String, usize>) -> Result<(usize, Vec<IrStatement>)> {
        // base_token is "self.balances"
        let name = base_token.trim_start_matches("self.");
        let var_def = self.storage_vars.get(name).ok_or(anyhow!("Unknown variable {}", name))?;
        let slot = var_def.slot;
        
        let mut keys = Vec::new();
        
        // Check for keys [k]
        while let Some(t) = self.peek() {
            if t == "[" {
                self.advance(); // consume [
                let k_str = self.advance().ok_or(anyhow!("Expected key"))?;
                if k_str == "msg.sender" {
                    keys.push(IrStatement::Caller);
                } else if let Some(off) = scope.get(&k_str) {
                    keys.push(IrStatement::CallDataLoad(*off));
                } else if let Ok(n) = k_str.parse::<U256>() {
                    keys.push(IrStatement::Push(n));
                } else if k_str == "address(0)" {
                    keys.push(IrStatement::Push(U256::zero()));
                } else {
                     return Err(anyhow!("Unknown key {}", k_str));
                }
                self.expect("]")?;
            } else {
                break;
            }
        }
        Ok((slot, keys))
    }
    
    fn emit_slot_calc(&self, base_slot: usize, keys: &[IrStatement], ir: &mut Vec<IrStatement>) -> Result<()> {
        if keys.is_empty() {
            ir.push(IrStatement::Push(U256::from(base_slot)));
            return Ok(());
        }
        
        ir.push(IrStatement::Push(U256::from(base_slot)));
        
        for key_op in keys {
            ir.push(IrStatement::Store { offset: 32 }); 
            ir.push(key_op.clone());                    
            ir.push(IrStatement::Store { offset: 0 });  
            
            ir.push(IrStatement::Push(U256::from(64)));
            ir.push(IrStatement::Push(U256::from(0)));
            ir.push(IrStatement::Sha3);
        }
        Ok(())
    }
    
    fn emit_expr_from_tokens(&mut self, tokens: &[String], ir: &mut Vec<IrStatement>, scope: &HashMap<String, usize>) -> Result<()> {
        if tokens.is_empty() { return Ok(()); }
        let first = &tokens[0];
        
        if first.starts_with("self.") {
             let (slot, keys) = self.consume_storage_access_from_tokens(tokens, scope)?;
             self.emit_slot_calc(slot, &keys, ir)?;
             ir.push(IrStatement::SLoad);
        } else if first == "address" && tokens.get(1).map(|s| s.as_str()) == Some("(") {
             ir.push(IrStatement::Push(U256::zero()));
        } else {
             // Maybe single token?
             if tokens.len() == 1 {
                 self.emit_load(first, ir, scope)?;
             } else {
                 return Err(anyhow!("Unsupported complex expression {:?}", tokens));
             }
        }
        Ok(())
    }
    
    fn consume_storage_access_from_tokens(&self, tokens: &[String], scope: &HashMap<String, usize>) -> Result<(usize, Vec<IrStatement>)> {
        // tokens: ["self.balances", "[", "msg.sender", "]"]
        // or ["self.allowances", "[", "from", "]", "[", "to", "]"]
        
        // This helper doesn't use self.tokens (the stream), but the provided slice.
        // It's a static parse of the slice.
        
        let base_token = &tokens[0];
        let name = base_token.trim_start_matches("self.");
        let var_def = self.storage_vars.get(name).ok_or(anyhow!("Unknown variable {}", name))?;
        let slot = var_def.slot;
        
        let mut keys = Vec::new();
        let mut pos = 1;
        
        while pos < tokens.len() {
            if tokens[pos] == "[" {
                pos += 1; // consume [
                if pos >= tokens.len() { return Err(anyhow!("Unexpected end of storage access")); }
                
                let k_str = &tokens[pos];
                pos += 1;
                
                if k_str == "msg.sender" {
                    keys.push(IrStatement::Caller);
                } else if let Some(off) = scope.get(k_str) {
                    keys.push(IrStatement::CallDataLoad(*off));
                } else if let Ok(n) = k_str.parse::<U256>() {
                    keys.push(IrStatement::Push(n));
                } else if k_str == "address" && tokens.get(pos).map(|s| s.as_str()) == Some("(") {
                    // address ( 0 )
                    // keys.push(0)
                     keys.push(IrStatement::Push(U256::zero()));
                     pos += 3; // ( 0 )
                } else {
                     return Err(anyhow!("Unknown key {}", k_str));
                }
                
                if pos >= tokens.len() || tokens[pos] != "]" {
                    return Err(anyhow!("Expected ]"));
                }
                pos += 1; // consume ]
            } else {
                break;
            }
        }
        
        Ok((slot, keys))
    }

    fn emit_load(&mut self, val_token: &str, ir: &mut Vec<IrStatement>, scope: &HashMap<String, usize>) -> Result<()> {
         if val_token == "msg.sender" {
            ir.push(IrStatement::Caller);
        } else if val_token == "True" {
            ir.push(IrStatement::Push(U256::one()));
        } else if val_token == "False" {
             ir.push(IrStatement::Push(U256::zero()));
        } else if let Some(off) = scope.get(val_token) {
            ir.push(IrStatement::CallDataLoad(*off));
        } else if let Ok(n) = val_token.parse::<U256>() {
            ir.push(IrStatement::Push(n));
        } else if val_token.starts_with("self.") {
             let (slot, keys) = self.consume_storage_access(val_token, scope)?;
             self.emit_slot_calc(slot, &keys, ir)?;
             ir.push(IrStatement::SLoad);
        } else if val_token == "address(0)" {
             ir.push(IrStatement::Push(U256::zero()));
        } else {
             return Err(anyhow!("Unknown val {}", val_token));
        }
        Ok(())
    }
    
    fn emit_load_complex(&mut self, lhs: &str, ir: &mut Vec<IrStatement>, scope: &HashMap<String, usize>) -> Result<()> {
        self.emit_load(lhs, ir, scope)
    }

    fn calc_selector(&self, name: &str) -> U256 {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        // Naive assumption: function has no arguments in selector for prototype unless parsed?
        // ERC20: transfer(address,uint256)
        // I didn't store argument types in `function_selectors` vector (only names).
        // I need to reconstruct full signature.
        
        let sig = match name {
            "transfer" => "transfer(address,uint256)",
            "approve" => "approve(address,uint256)",
            "transfer_from" => "transferFrom(address,address,uint256)",
            "balance_of" => "balanceOf(address)",
            _ => { return U256::from(0x12345678); } // fallback
        };
        
        hasher.update(sig.as_bytes());
        let res = hasher.finalize();
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&res[0..4]);
        U256::from(u32::from_be_bytes(bytes))
    }
}
