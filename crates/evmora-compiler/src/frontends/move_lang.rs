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

    fn compile_to_ir(&self, source: &str) -> Result<IrProgram> {
        let mut program = IrProgram::new();
        
        program.statements.push(IrStatement::CallDataLoad(0));
        program.statements.push(IrStatement::Push(U256::from(224)));

        // Move: public fun name(args)
        let re = regex::Regex::new(r"public\s+fun\s+(?P<name>\w+)\s*\(").unwrap();
        let mut found_funcs = false;

        for cap in re.captures_iter(source) {
            found_funcs = true;
            let func_name = &cap["name"];
            
            program.statements.push(IrStatement::Label(func_name.to_string()));
            
            // Logic skeleton
            program.statements.push(IrStatement::Push(U256::one()));
            program.statements.push(IrStatement::Push(U256::zero()));
            program.statements.push(IrStatement::Store { offset: 0 });
            program.statements.push(IrStatement::Return { offset: 0, size: 32 });
        }
        
        // Default fallback
        if !found_funcs {
             program.statements.push(IrStatement::Stop);
        }
        
        Ok(program)
    }
}
