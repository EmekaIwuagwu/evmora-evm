
pub struct Codegen;

impl Codegen {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, ir: &crate::ir::IrProgram) -> Vec<u8> {
        use std::collections::HashMap;
        
        // Pass 1: Calculate offsets
        let mut labels = HashMap::new();
        let mut pc = 0usize;
        
        for stmt in &ir.statements {
            match stmt {
                crate::ir::IrStatement::Push(_) => pc += 33, // PUSH32 + 32 bytes
                crate::ir::IrStatement::Pop |
                crate::ir::IrStatement::Add |
                crate::ir::IrStatement::Sub |
                crate::ir::IrStatement::Mul |
                crate::ir::IrStatement::Div |
                crate::ir::IrStatement::Shr |
                crate::ir::IrStatement::Shl |
                crate::ir::IrStatement::Sha3 |
                crate::ir::IrStatement::SLoad |
                crate::ir::IrStatement::SStore |
                crate::ir::IrStatement::Caller |
                crate::ir::IrStatement::CallValue | 
                crate::ir::IrStatement::Eq | 
                crate::ir::IrStatement::And | 
                crate::ir::IrStatement::Or | 
                crate::ir::IrStatement::Not | 
                crate::ir::IrStatement::IsZero |
                crate::ir::IrStatement::Stop => pc += 1,
                
                crate::ir::IrStatement::Dup(_) => pc += 1,
                crate::ir::IrStatement::Swap(_) => pc += 1,

                crate::ir::IrStatement::Store { .. } => pc += 34, // PUSH32 (33) + MSTORE (1)
                crate::ir::IrStatement::Load { .. } => pc += 34, // PUSH32 (33) + MLOAD (1)
                crate::ir::IrStatement::CallDataLoad(_) => pc += 34, // PUSH32 (33) + CALLDATALOAD (1)
                crate::ir::IrStatement::Return { .. } => pc += 67, // PUSH (33) + PUSH (33) + RETURN (1)
                crate::ir::IrStatement::Revert { .. } => pc += 67, 
                
                crate::ir::IrStatement::Jump(_) => pc += 34, // PUSH32 (dest) + JUMP
                crate::ir::IrStatement::JumpI(_) => pc += 34, // PUSH32 (dest) + JUMPI
                crate::ir::IrStatement::Label(name) => {
                    labels.insert(name.clone(), pc);
                    pc += 1; // JUMPDEST
                },
                crate::ir::IrStatement::RawBytecode(bytes) => pc += bytes.len(),
                _ => {} // FunctionCall handles nested? Or unimplemented
            }
        }
        
        // Pass 2: Generate
        let mut bytecode = Vec::new();
        
        for stmt in &ir.statements {
            match stmt {
                crate::ir::IrStatement::Push(val) => {
                    bytecode.push(0x7f); // PUSH32
                    let mut bytes = [0u8; 32];
                    val.to_big_endian(&mut bytes);
                    bytecode.extend_from_slice(&bytes);
                }
                crate::ir::IrStatement::Pop => bytecode.push(0x50),
                crate::ir::IrStatement::Add => bytecode.push(0x01),
                crate::ir::IrStatement::Sub => bytecode.push(0x03),
                crate::ir::IrStatement::Mul => bytecode.push(0x02),
                crate::ir::IrStatement::Div => bytecode.push(0x04),
                crate::ir::IrStatement::Shr => bytecode.push(0x1c),
                crate::ir::IrStatement::Shl => bytecode.push(0x1b),
                crate::ir::IrStatement::Sha3 => bytecode.push(0x20),
                crate::ir::IrStatement::SLoad => bytecode.push(0x54),
                crate::ir::IrStatement::SStore => bytecode.push(0x55),
                crate::ir::IrStatement::Eq => bytecode.push(0x14),
                crate::ir::IrStatement::IsZero => bytecode.push(0x15),
                crate::ir::IrStatement::And => bytecode.push(0x16),
                crate::ir::IrStatement::Or => bytecode.push(0x17),
                crate::ir::IrStatement::Not => bytecode.push(0x19),
                crate::ir::IrStatement::Stop => bytecode.push(0x00),
                crate::ir::IrStatement::CallValue => bytecode.push(0x34),
                crate::ir::IrStatement::Caller => bytecode.push(0x33),

                crate::ir::IrStatement::Dup(n) => {
                    // Valid n is 1..=16
                    bytecode.push(0x80 + (n - 1));
                },
                crate::ir::IrStatement::Swap(n) => {
                    // Valid n is 1..=16
                    bytecode.push(0x90 + (n - 1));
                },

                crate::ir::IrStatement::Store { offset } => {
                    Self::push_val(&mut bytecode, *offset);
                    bytecode.push(0x52);
                },
                crate::ir::IrStatement::Load { offset } => {
                    Self::push_val(&mut bytecode, *offset);
                    bytecode.push(0x51);
                },
                crate::ir::IrStatement::CallDataLoad(offset) => {
                    Self::push_val(&mut bytecode, *offset);
                    bytecode.push(0x35);
                },
                crate::ir::IrStatement::Return { offset, size } => {
                    Self::push_val(&mut bytecode, *offset);
                    Self::push_val(&mut bytecode, *size);
                    bytecode.push(0xf3);
                },
                crate::ir::IrStatement::Revert { offset, size } => {
                    Self::push_val(&mut bytecode, *offset);
                    Self::push_val(&mut bytecode, *size);
                    bytecode.push(0xfd);
                },
                
                crate::ir::IrStatement::Jump(label) => {
                    let dest = *labels.get(label).unwrap_or(&0);
                    Self::push_val(&mut bytecode, dest);
                    bytecode.push(0x56);
                }, 
                crate::ir::IrStatement::JumpI(label) => {
                    let dest = *labels.get(label).unwrap_or(&0);
                    Self::push_val(&mut bytecode, dest);
                    bytecode.push(0x57);
                }, 
                crate::ir::IrStatement::Label(_) => bytecode.push(0x5b), 
                crate::ir::IrStatement::RawBytecode(bytes) => bytecode.extend_from_slice(bytes),
                _ => {}
            }
        }
        
        bytecode
    }

    fn push_val(bytecode: &mut Vec<u8>, val: usize) {
        bytecode.push(0x7f); // PUSH32
        let mut bytes = [0u8; 32];
        let val_bytes = val.to_be_bytes(); // 8 bytes
        // Copy to end
        bytes[24..32].copy_from_slice(&val_bytes);
        bytecode.extend_from_slice(&bytes);
    }
}
