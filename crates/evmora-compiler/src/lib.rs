pub mod parser;
pub mod codegen;
pub mod optimizer;
pub mod yul;
pub mod ir;
pub mod frontends;

use frontends::{CompilerFrontend, QuorlinFrontend, SolidityFrontend, VyperFrontend, MoveFrontend};
use codegen::Codegen;
use anyhow::{Result, anyhow};

pub struct Compiler {
    frontends: Vec<Box<dyn CompilerFrontend>>,
    codegen: Codegen,
}

impl Compiler {
    pub fn new() -> Self {
        let mut frontends: Vec<Box<dyn CompilerFrontend>> = Vec::new();
        frontends.push(Box::new(QuorlinFrontend));
        frontends.push(Box::new(SolidityFrontend));
        frontends.push(Box::new(VyperFrontend));
        frontends.push(Box::new(MoveFrontend));
        
        Self {
            frontends,
            codegen: Codegen::new(),
        }
    }
    
    pub fn compile(&self, source: &str, file_extension: &str) -> Result<Vec<u8>> {
        let frontend = self.frontends.iter()
            .find(|f| f.extension() == file_extension)
            .ok_or_else(|| anyhow!("No frontend found for extension .{}", file_extension))?;
            
        let ir = frontend.compile_to_ir(source)?;
        Ok(self.codegen.generate(&ir))
    }
}

