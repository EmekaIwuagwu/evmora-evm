use anyhow::Result;
use crate::ir::IrProgram;

pub trait CompilerFrontend: Send + Sync {
    fn name(&self) -> &str;
    fn extension(&self) -> &str;
    fn compile_to_ir(&self, source: &str) -> Result<IrProgram>;
}
