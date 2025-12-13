use evmora_compiler::frontends::solidity::SolidityFrontend;
use evmora_compiler::frontends::traits::CompilerFrontend;
use evmora_compiler::frontends::quorlin::QuorlinFrontend;
use evmora_compiler::frontends::vyper::VyperFrontend;

#[test]
fn test_solidity_compiler_check() {
    let frontend = SolidityFrontend::new();
    println!("Testing Solidity compiler bridge...");
    let result = frontend.compile_to_ir("contract A {}", None);
    
    match result {
        Ok(_) => println!("Solc found and compilation succeeded"),
        Err(e) => {
            let msg = e.to_string();
            println!("Solc bridge execution result: {}", msg);
        }
    }
}

#[test]
fn test_vyper_compiler_check() {
    let frontend = VyperFrontend::new();
    println!("Testing Vyper compiler bridge...");
    // Vyper needs version pragma usually but handled by wrapper? Bridge just passes compilation.
    let result = frontend.compile_to_ir("# @version ^0.3.0\n", None);
     match result {
        Ok(_) => println!("Vyper found and compilation succeeded"),
        Err(e) => {
             let msg = e.to_string();
             println!("Vyper bridge execution result: {}", msg);
        }
    }
}

#[test]
fn test_quorlin_compiler_real() {
    let frontend = QuorlinFrontend;
    println!("Testing Quorlin compiler (real implementation)...");
    
    // Test the mini-compiler grammar
    let source = r#"
    contract Counter {
        uint256 count;

        fn increment() {
             count += 1;
        }
    }
    "#;
    
    let result = frontend.compile_to_ir(source, None);
    match result {
        Ok(program) => {
             println!("Quorlin compilation succeeded!");
             println!("Generated {} instructions", program.statements.len());
             assert!(program.statements.len() > 10, "Should generate dispatch and logic instructions");
        },
        Err(e) => {
             panic!("Quorlin compilation failed: {}", e);
        }
    }
}
