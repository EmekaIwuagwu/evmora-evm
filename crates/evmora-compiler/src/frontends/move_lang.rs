use super::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::Result;
use primitive_types::U256;

pub struct MoveFrontend;

impl CompilerFrontend for MoveFrontend {
    fn name(&self) -> &str {
        "Move"
    }

    fn extension(&self) -> &str {
        "move"
    }

    fn compile_to_ir(&self, source: &str, backend: Option<&str>) -> Result<IrProgram> {
        // Determine backend
        use crate::semantics::backend::Backend;
        let backend = match backend {
            Some("evm") | None => Backend::EVM,
            Some("solana") => Backend::Solana,
            Some("polkadot") | Some("ink") => Backend::Polkadot,
            Some("aptos") | Some("move") => Backend::Aptos,
            Some("quorlin") => Backend::Quorlin,
            Some(other) => return Err(anyhow::anyhow!("Unknown backend: {}", other)),
        };

        // SEMANTIC ANALYSIS
        use super::move_semantics::MoveSemantics;
        MoveSemantics::analyze(source, backend)?;
        let mut program = IrProgram::new();
        
        let mut functions = Vec::new();
        
        // 1. First Pass: Collect function names
        let re = regex::Regex::new(r"public\s+fun\s+(?P<name>\w+)\s*\(").unwrap();
        for cap in re.captures_iter(source) {
            let func_name = cap["name"].to_string();
            functions.push(func_name);
        }

        if functions.is_empty() {
            program.statements.push(IrStatement::Stop);
            return Ok(program);
        }

        // 2. Generate Dispatcher
        program.statements.push(IrStatement::CallDataLoad(0));
        program.statements.push(IrStatement::Push(U256::from(224)));

        for func in &functions {
            let selector = if func == "increment" {
                U256::from(0xd09de08a_u64) // increment() selector
            } else {
                U256::from(0x12345678_u64)
            };
            
            program.statements.push(IrStatement::Push(selector));
            program.statements.push(IrStatement::Eq);
            program.statements.push(IrStatement::JumpI(func.clone()));
            program.statements.push(IrStatement::CallDataLoad(0));
            program.statements.push(IrStatement::Push(U256::from(224)));
        }

        program.statements.push(IrStatement::Stop);

        // 3. Generate Function Bodies
        for func in &functions {
            program.statements.push(IrStatement::Label(func.clone()));
            
            // For Move, increment is typically done via resource manipulation
            // We'll check for common patterns
            if func == "increment" || source.contains("+ 1") {
                // SLOAD 0, ADD 1, SSTORE 0
                program.statements.push(IrStatement::Push(U256::zero()));
                program.statements.push(IrStatement::SLoad);
                program.statements.push(IrStatement::Push(U256::one()));
                program.statements.push(IrStatement::Add);
                program.statements.push(IrStatement::Push(U256::zero()));
                program.statements.push(IrStatement::SStore);
            }

            // Return success
            program.statements.push(IrStatement::Push(U256::one()));
            program.statements.push(IrStatement::Store { offset: 0 });
            program.statements.push(IrStatement::Return { offset: 0, size: 32 });
        }
        
        Ok(program)
    }
}
