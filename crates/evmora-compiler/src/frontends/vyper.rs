use super::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::Result;
use primitive_types::U256;

pub struct VyperFrontend;

impl CompilerFrontend for VyperFrontend {
    fn name(&self) -> &str {
        "Vyper"
    }

    fn extension(&self) -> &str {
        "vy"
    }

    fn compile_to_ir(&self, source: &str) -> Result<IrProgram> {
        let mut program = IrProgram::new();
        
        program.statements.push(IrStatement::CallDataLoad(0));
        program.statements.push(IrStatement::Push(U256::from(224)));

        // Vyper: @external followed by def name(args):
        let re = regex::Regex::new(r"def\s+(?P<name>\w+)\s*\(").unwrap();
        let mut found_funcs = false;

        for cap in re.captures_iter(source) {
            found_funcs = true;
            let func_name = &cap["name"];
            if func_name == "__init__" { continue; }

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
