pub mod parser;
pub mod codegen;
pub mod optimizer;
pub mod yul;
pub mod ir;
pub mod frontends;

use frontends::{CompilerFrontend, QuorlinFrontend, SolidityFrontend, VyperFrontend, MoveFrontend};
use codegen::Codegen;
use anyhow::{Result, anyhow, Context};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOpts {
    pub language: Option<String>,
    pub target: Option<String>,
    pub deterministic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub bytecode: String, // Hex string
    pub abi: Option<serde_json::Value>,
    pub ir: Option<serde_json::Value>,
    pub build_info: BuildInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfo {
    pub compiler_version: String,
    pub timestamp: String,
    pub opts: CompileOpts,
}

pub struct Compiler {
    frontends: Vec<Box<dyn CompilerFrontend>>,
    codegen: Codegen,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
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
    
    // Kept for backward compatibility if needed, but compile_file is preferred
    pub fn compile(&self, source: &str, file_extension: &str) -> Result<Vec<u8>> {
        let frontend = self.frontends.iter()
            .find(|f| f.extension() == file_extension)
            .ok_or_else(|| anyhow!("No frontend found for extension .{}", file_extension))?;
            
        let ir = frontend.compile_to_ir(source, None)?;
        Ok(self.codegen.generate(&ir))
    }

    pub fn compile_file(&self, path: &str, opts: CompileOpts) -> Result<Artifact> {
        let path_obj = Path::new(path);
        let extension = path_obj.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
            
        // If language explicitly provided, use it. Otherwise use extension.
        // Also map extensions to standard lang names if needed, but frontends verify extension.
        // Here we search by extension mostly.
        
        let target_lang_or_ext = opts.language.as_deref().unwrap_or(extension);
        
        // Find frontend by name (sol, ql, etc) OR extension (sol, ql, etc)
        let frontend = self.frontends.iter()
            .find(|f| f.extension() == target_lang_or_ext || f.name().eq_ignore_ascii_case(target_lang_or_ext))
            .ok_or_else(|| anyhow!("No frontend found for language/extension '{}'", target_lang_or_ext))?;
            
        let source = fs::read_to_string(path).context(format!("Failed to read source file: {}", path))?;
        let ir = frontend.compile_to_ir(&source, opts.target.as_deref())?;
        
        // Deterministic generation
        // For now, our simple codegen is deterministic, but we'd pass flags here if needed.
        let bytecode_bytes = self.codegen.generate(&ir);
        let bytecode_hex = hex::encode(bytecode_bytes);
        
        // Mock ABI for now as our IR doesn't fully capture ABI yet
        let abi = serde_json::json!([]);

         // IR dump - specific frontends might provide better IR dumps, but here we dump the IrProgram
        let ir_json = serde_json::to_value(format!("{:?}", ir)).unwrap_or(serde_json::Value::Null);

        Ok(Artifact {
            bytecode: bytecode_hex,
            abi: Some(abi),
            ir: Some(ir_json),
            build_info: BuildInfo {
                compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: if opts.deterministic { "1970-01-01T00:00:00Z".to_string() } else { chrono::Utc::now().to_rfc3339() },
                opts,
            }
        })
    }
}

