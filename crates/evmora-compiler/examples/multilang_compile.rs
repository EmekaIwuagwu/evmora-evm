use evmora_compiler::{Compiler, CompileOpts};
use anyhow::Result;

fn main() -> Result<()> {
    let compiler = Compiler::new();
    let opts = CompileOpts { language: None, deterministic: true };
    
    // Project root relative paths
    let paths = vec![
        ("tests/fixtures/ql/Counter.ql", "ql"),
        ("tests/fixtures/sol/Counter.sol", "sol"),
        ("tests/fixtures/vy/Counter.vy", "vy"),
        ("tests/fixtures/move/Counter.move", "move"),
    ];

    for (path, label) in paths {
        println!("\n--- {} Compilation ---", label);
        if !std::path::Path::new(path).exists() {
             println!("Skipping {}, file not found", path);
             continue;
        }
        
        match compiler.compile_file(path, opts.clone()) {
            Ok(artifact) => {
                println!("Success! Bytecode length: {}", artifact.bytecode.len() / 2);
                println!("Bytecode: {}", artifact.bytecode);
            },
            Err(e) => {
                println!("Failed: {}", e);
            }
        }
    }

    Ok(())
}
