use super::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::Result;
use primitive_types::U256;

pub struct SolidityFrontend;

impl CompilerFrontend for SolidityFrontend {
    fn name(&self) -> &str {
        "Solidity"
    }

    fn extension(&self) -> &str {
        "sol"
    }

    fn compile_to_ir(&self, source: &str) -> Result<IrProgram> {
        let mut program = IrProgram::new();
        
        // 1. Dispatcher setup: Load Selector
        program.statements.push(IrStatement::CallDataLoad(0));
        program.statements.push(IrStatement::Push(U256::from(224)));  // SHR 224 (mock logic)
        
        // Dynamic Parsing using Regex
        // Matches: function name(args...) ...
        let re = regex::Regex::new(r"function\s+(?P<name>\w+)\s*\(").unwrap();
        
        let mut found_funcs = false;
        
        for cap in re.captures_iter(source) {
            found_funcs = true;
            let func_name = &cap["name"];
            
            // Skip constructor
            if func_name == "constructor" { continue; }
            
            // In a real compiler, we'd hash the signature "name(type,type)" to get the selector.
            // Here we just create a Label for it.
            program.statements.push(IrStatement::Label(func_name.to_string()));
            
            // Mock Body extraction (very naive: assume return 1 for success or boolean)
            program.statements.push(IrStatement::Push(U256::one()));
            program.statements.push(IrStatement::Push(U256::zero()));
            program.statements.push(IrStatement::Store { offset: 0 });
            program.statements.push(IrStatement::Return { offset: 0, size: 32 });
        }

        if !found_funcs {
             // Fallback if no specific functions found
             program.statements.push(IrStatement::Push(U256::from(1)));
             program.statements.push(IrStatement::Push(U256::from(0)));
             program.statements.push(IrStatement::Store { offset: 0 });
             program.statements.push(IrStatement::Return { offset: 0, size: 32 });
        }
        
        Ok(program)
    }
}
