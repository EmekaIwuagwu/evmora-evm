use crate::frontends::traits::CompilerFrontend;
use crate::ir::{IrProgram, IrStatement};
use anyhow::{anyhow, Context, Result};
use std::io::Write;
use std::process::Command;
use tempfile::Builder;

pub struct SolidityFrontend;

impl SolidityFrontend {
    pub fn new() -> Self {
        Self
    }

    fn find_solc(&self) -> Result<std::path::PathBuf> {
        // Try to find solc in PATH
        which::which("solc").or_else(|_| {
            // Check common locations or env var
            std::env::var("SOLC_PATH").map(std::path::PathBuf::from).map_err(|_| anyhow!("solc not found"))
        }).context("Solidity compiler (solc) not found. Please install solc or set SOLC_PATH.")
    }
}

impl CompilerFrontend for SolidityFrontend {
    fn name(&self) -> &str {
        "Solidity"
    }

    fn extension(&self) -> &str {
        "sol"
    }

    fn compile_to_ir(&self, source: &str, _backend: Option<&str>) -> Result<IrProgram> {
        // 1. Check if solc exists
        let solc_path = self.find_solc()?;

        // 2. Create temp file
        let mut temp_file = Builder::new()
            .suffix(".sol")
            .tempfile()
            .context("Failed to create temp file for solidity source")?;

        write!(temp_file, "{}", source).context("Failed to write source code to temp file")?;
        let temp_path = temp_file.path().to_owned();

        // 3. Run solc
        // solc --bin --optimize <file>
        let output = Command::new(solc_path)
            .arg("--bin")
            .arg("--optimize") // Always optimize for now
            .arg(temp_path)
            .output()
            .context("Failed to execute solc")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Solidity compilation failed:\n{}", stderr));
        }

        // 4. Parse output
        // Output format is usually:
        //
        // ======= <file>:<ContractName> =======
        // Binary: 
        // <hex string>
        
        // We'll just grab the hex string. If there are multiple contracts, solc outputs multiple sections.
        // For this simple bridge, we'll take the LAST contract's bytecode (often the main one if inherited) 
        // or just strict parsing.
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Simple parser for solc --bin output
        let mut bytecode_hex = String::new();
        let mut found_binary_label = false;
        
        for line in stdout.lines() {
            if line.starts_with("Binary:") {
                found_binary_label = true;
                continue;
            }
            if found_binary_label && !line.trim().is_empty() {
                bytecode_hex = line.trim().to_string();
                // We keep going to get the last one if multiple? 
                // Usually for a single file test it's fine.
                // Reset flag to capture next if any
                found_binary_label = false; 
            }
        }
        
        if bytecode_hex.is_empty() {
             // Fallback: maybe it just printed the hex? (unlikely with --bin)
             return Err(anyhow!("Could not parse bytecode from solc output"));
        }

        let bytecode = hex::decode(&bytecode_hex)
            .context(format!("Failed to decode bytecode hex: {}", bytecode_hex))?;

        // 5. Wrap in IrProgram
        let mut program = IrProgram::new();
        program.statements.push(IrStatement::RawBytecode(bytecode));

        Ok(program)
    }
}
