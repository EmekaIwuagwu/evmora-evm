use crate::ir::IrProgram;
use std::collections::HashMap;

pub struct MoveCodegen;

impl MoveCodegen {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, ir: &IrProgram) -> Vec<u8> {
        // Generate Move bytecode from IR
        // Move bytecode format: https://github.com/move-language/move/blob/main/language/move-binary-format/src/file_format.rs
        
        let mut bytecode = Vec::new();
        
        // Move module header
        // Magic number for Move bytecode
        bytecode.extend_from_slice(&[0xA1, 0x1C, 0xEB, 0x0B]); // Move magic
        bytecode.push(6); // Version
        
        // Module header
        bytecode.push(0x01); // Module
        
        // Address pool (simplified)
        bytecode.push(0x01); // 1 address
        bytecode.extend_from_slice(&[0; 16]); // 0x0 address
        
        // Identifier pool
        bytecode.push(0x02); // 2 identifiers
        // "main"
        bytecode.push(4);
        bytecode.extend_from_slice(b"main");
        // "value"
        bytecode.push(5);
        bytecode.extend_from_slice(b"value");
        
        // Function signatures
        bytecode.push(0x01); // 1 signature
        bytecode.push(0x00); // return type: ()
        bytecode.push(0x00); // no params
        
        // Function definitions
        bytecode.push(0x01); // 1 function
        bytecode.push(0x00); // function 0
        bytecode.push(0x00); // visibility: private
        bytecode.push(0x00); // acquires: none
        
        // Code section
        bytecode.push(0x01); // 1 code unit
        
        // Generate bytecode from IR statements
        let mut code = Vec::new();
        let mut locals_count = 0u8;
        
        for stmt in &ir.statements {
            match stmt {
                crate::ir::IrStatement::Push(val) => {
                    // LdU64 or LdU128
                    if val.bits() <= 64 {
                        code.push(0x08); // LdU64
                        code.extend_from_slice(&val.low_u64().to_le_bytes());
                    } else {
                        code.push(0x09); // LdU128
                        code.extend_from_slice(&val.low_u128().to_le_bytes());
                    }
                }
                crate::ir::IrStatement::Add => {
                    code.push(0x1F); // Add
                }
                crate::ir::IrStatement::Sub => {
                    code.push(0x20); // Sub
                }
                crate::ir::IrStatement::Pop => {
                    code.push(0x03); // Pop
                }
                crate::ir::IrStatement::Return { .. } => {
                    code.push(0x05); // Ret
                }
                _ => {
                    // Handle other IR statements
                }
            }
        }
        
        // Add final return if not present
        if code.is_empty() || code.last() != Some(&0x05) {
            code.push(0x05); // Ret
        }
        
        // Write locals count
        bytecode.push(locals_count);
        
        // Write code length and code
        let code_len = code.len() as u16;
        bytecode.extend_from_slice(&code_len.to_le_bytes());
        bytecode.extend_from_slice(&code);
        
        bytecode
    }
}

