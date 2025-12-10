use hex;

pub struct Codegen;

impl Codegen {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, ir: &crate::ir::IrProgram) -> Vec<u8> {
        // Translate IR to EVM bytecode
        let mut bytecode = Vec::new();
        
        for stmt in &ir.statements {
            match stmt {
                crate::ir::IrStatement::Push(val) => {
                    // Naive PUSH32 for everything for simplicity
                    bytecode.push(0x7f); // PUSH32
                    let mut bytes = [0u8; 32];
                    val.to_big_endian(&mut bytes);
                    bytecode.extend_from_slice(&bytes);
                }
                crate::ir::IrStatement::Pop => bytecode.push(0x50),
                crate::ir::IrStatement::Add => bytecode.push(0x01),
                crate::ir::IrStatement::Sub => bytecode.push(0x03),
                crate::ir::IrStatement::Sha3 => bytecode.push(0x20),
                crate::ir::IrStatement::SLoad => bytecode.push(0x54),
                crate::ir::IrStatement::SStore => bytecode.push(0x55),
                crate::ir::IrStatement::Store { .. } => bytecode.push(0x52), // MSTORE (ignoring offset arg for naive gen)
                crate::ir::IrStatement::Load { .. } => bytecode.push(0x51), // MLOAD
                crate::ir::IrStatement::CallDataLoad(_) => bytecode.push(0x35),
                crate::ir::IrStatement::Return { .. } => bytecode.push(0xf3),
                crate::ir::IrStatement::Caller => bytecode.push(0x33),
                _ => { 
                    // Placeholder for jump labels, etc.
                    bytecode.push(0x00); // STOP
                }
            }
        }
        
        bytecode
    }
}
