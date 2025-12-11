use clap::{Parser, Subcommand};
use evmora_compiler::{Compiler, CompileOpts, Artifact};
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Parser)]
#[command(name = "evmora-compiler")]
#[command(about = "Multi-language EVM compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a source file
    Compile {
        /// Source file path
        path: String,

        /// Output directory for artifacts
        #[arg(long, default_value = "target/evmora/artifacts")]
        out: String,

        /// Source language (sol, ql, vy, move). If not provided, inferred from extension.
        #[arg(long)]
        lang: Option<String>,

        /// Target backend (evm, solana, polkadot, aptos, quorlin). Default: evm
        #[arg(long, default_value = "evm")]
        target: String,

        /// Enable deterministic build (pinned timestamp etc)
        #[arg(long, default_value_t = false)]
        deterministic: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { path, out, lang, target, deterministic } => {
            let compiler = Compiler::new();
            let opts = CompileOpts {
                language: lang,
                target: Some(target),
                deterministic,
            };

            println!("Compiling {}...", path);
            let artifact = compiler.compile_file(&path, opts)?;
            
            save_artifact(&path, &out, &artifact)?;
            println!("Compilation successful. Artifacts saved to {}", out);
        }
    }

    Ok(())
}

fn save_artifact(source_path: &str, out_dir: &str, artifact: &Artifact) -> Result<()> {
    let source_path = Path::new(source_path);
    let stem = source_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("contract");
    
    // Determine language folder if possible, or just use 'unknown'
    let lang = artifact.build_info.opts.language.clone()
        .or_else(|| source_path.extension().and_then(|e| e.to_str()).map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    let artifact_dir = Path::new(out_dir).join(stem).join(&lang);
    fs::create_dir_all(&artifact_dir).context("Failed to create artifact directory")?;

    // Save bytecode.bin
    let bytecode_path = artifact_dir.join("bytecode.bin");
    fs::write(&bytecode_path, &artifact.bytecode).context("Failed to write bytecode")?;

    // Save abi.json
    if let Some(abi) = &artifact.abi {
        let abi_path = artifact_dir.join("abi.json");
        let abi_str = serde_json::to_string_pretty(abi)?;
        fs::write(abi_path, abi_str)?;
    }

    // Save ir.json
    if let Some(ir) = &artifact.ir {
        let ir_path = artifact_dir.join("ir.json");
        let ir_str = serde_json::to_string_pretty(ir)?;
        fs::write(ir_path, ir_str)?;
    }

    // Save build-info.json
    let build_info_path = artifact_dir.join("build-info.json");
    let build_info_str = serde_json::to_string_pretty(&artifact.build_info)?;
    fs::write(build_info_path, build_info_str)?;

    Ok(())
}
