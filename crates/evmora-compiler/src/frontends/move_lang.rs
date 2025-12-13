use super::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::io::Write;
use tempfile::Builder;
use std::fs;

pub struct MoveFrontend;

impl MoveFrontend {
    pub fn new() -> Self {
        Self
    }
    
    fn find_move(&self) -> Result<std::path::PathBuf> {
        // Look for 'move' or 'aptos' CLI
        which::which("aptos").or_else(|_| which::which("move"))
            .context("Move compiler not found. Please install 'aptos' CLI or 'move' CLI.")
    }
}

impl CompilerFrontend for MoveFrontend {
    fn name(&self) -> &str { "Move" }
    fn extension(&self) -> &str { "move" }

    fn compile_to_ir(&self, source: &str, _backend: Option<&str>) -> Result<IrProgram> {
        let move_cli = self.find_move()?;
        
        // Move requires a package structure:
        // temp_dir/
        //   Move.toml
        //   sources/
        //     Contract.move
        
        let temp_dir = Builder::new().prefix("move_build").tempdir()?;
        let sources_dir = temp_dir.path().join("sources");
        fs::create_dir(&sources_dir)?;
        
        // Write source file
        let file_path = sources_dir.join("Contract.move");
        fs::write(&file_path, source)?;
        
        // Write minimal Move.toml
        let move_toml = r#"
[package]
name = "TempPackage"
version = "0.0.1"
"#;
        fs::write(temp_dir.path().join("Move.toml"), move_toml)?;
        
        // Run build command
        // aptos move build --package-dir <path>
        // OR move build -p <path>
        // We'll try generic arguments that might work for both or detect
        
        let exe_name = move_cli.file_name().unwrap().to_string_lossy();
        let mut cmd = Command::new(&move_cli);
        
        if exe_name.contains("aptos") {
            cmd.arg("move").arg("compile").arg("--package-dir").arg(temp_dir.path());
        } else {
            cmd.arg("build").arg("-p").arg(temp_dir.path());
        }
        
        let output = cmd.output().context("Failed to execute move compiler")?;
        
        if !output.status.success() {
             return Err(anyhow!("Move compilation failed:\n{}", String::from_utf8_lossy(&output.stderr)));
        }
        
        // Compilation successful. Now find the bytecode.
        // Usually in build/TempPackage/bytecode_modules/Contract.mv
        let build_dir = temp_dir.path().join("build").join("TempPackage").join("bytecode_modules");
        
        // Find first .mv file
        let mut bytecode = Vec::new();
        if build_dir.exists() {
            for entry in fs::read_dir(build_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "mv") {
                    bytecode = fs::read(path)?;
                    break;
                }
            }
        }
        
        if bytecode.is_empty() {
             return Err(anyhow!("Could not find compiled bytecode in output directory"));
        }
        
        // Wrap in IrProgram
        let mut program = IrProgram::new();
        program.statements.push(IrStatement::RawBytecode(bytecode));
        
        Ok(program)
    }
}
