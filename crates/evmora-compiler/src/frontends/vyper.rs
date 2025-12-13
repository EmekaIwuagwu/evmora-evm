use crate::frontends::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::{anyhow, Context, Result};
use std::io::Write;
use std::process::Command;
use tempfile::Builder;

pub struct VyperFrontend;

impl VyperFrontend {
    pub fn new() -> Self {
        Self
    }

    fn find_vyper(&self) -> Result<std::path::PathBuf> {
        which::which("vyper").or_else(|_| {
             std::env::var("VYPER_PATH").map(std::path::PathBuf::from).map_err(|_| anyhow!("vyper not found"))
        }).context("Vyper compiler (vyper) not found. Please install vyper or set VYPER_PATH.")
    }
}

impl CompilerFrontend for VyperFrontend {
    fn name(&self) -> &str {
        "Vyper"
    }

    fn extension(&self) -> &str {
        "vy"
    }

    fn compile_to_ir(&self, source: &str, _backend: Option<&str>) -> Result<IrProgram> {
        let vyper_path = self.find_vyper()?;

        // Write source to temp file
        let mut temp_file = Builder::new()
            .suffix(".vy")
            .tempfile()
            .context("Failed to create temp file for vyper source")?;

        write!(temp_file, "{}", source).context("Failed to write source code to temp file")?;
        let temp_path = temp_file.path().to_owned();

        // Run vyper
        // vyper -f bytecode <file>
        let output = Command::new(vyper_path)
            .arg("-f")
            .arg("bytecode")
            .arg(temp_path)
            .output()
            .context("Failed to execute vyper")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Vyper compilation failed:\n{}", stderr));
        }

        let bytecode_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // Remove 0x prefix if present
        let bytecode_hex = bytecode_hex.trim_start_matches("0x");

        if bytecode_hex.is_empty() {
            return Err(anyhow!("Vyper returned empty bytecode"));
        }

        let bytecode = hex::decode(bytecode_hex)
            .context("Failed to decode bytecode hex from vyper output")?;

        let mut program = IrProgram::new();
        program.statements.push(IrStatement::RawBytecode(bytecode));

        Ok(program)
    }
}
