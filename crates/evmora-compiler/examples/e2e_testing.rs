use evmora_compiler::Compiler;
use evmora_runtime::EvmClient;
use evmora_core::types::{Transaction, Address};
use primitive_types::U256;
use hex;

// tokio is not a dependency of evmora-compiler yet, using sync runtime or just block_on stub
// But evm client is sync in some parts, async in others?
// Client execute is async.
// We need a runtime.

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    println!("=== Evmora End-to-End Workflow Test ===");
    
    // 1. Initialize System
    let compiler = Compiler::new();
    let mut client = EvmClient::new("config.toml")?;
    println!("[+] System Initialized");
    
    // 2. Load Contract Source (Quorlin)
    let source_path = "tests/fixtures/contracts/token.ql";
    let source_code = std::fs::read_to_string(source_path)?;
    println!("[+] Loaded Quorlin Source: {} bytes", source_code.len());
    
    // 3. Compile
    let bytecode = compiler.compile(&source_code, "ql")?;
    println!("[+] Compiled to Bytecode: {} bytes", bytecode.len());
    println!("    Sample: {}", hex::encode(&bytecode[0..std::cmp::min(20, bytecode.len())]));
    
    // 4. Deploy Transaction
    println!("[+] deploying contract...");
    let deploy_tx = Transaction::create(
        bytecode.clone(),
        vec![], // No constructor args for this naive test
        1_000_000 // Huge gas limit for safety
    );
    
    let deploy_res = client.execute(deploy_tx).await?;
    
    if !deploy_res.success {
        panic!("[-] Deployment Failed! Gas Used: {}", deploy_res.gas_used);
    }
    
    let contract_addr = deploy_res.contract_address.unwrap();
    println!("[+] Deployment Successful!");
    println!("    Address: {:?}", contract_addr);
    println!("    Gas Used: {}", deploy_res.gas_used);
    println!("    Time: {:?}", deploy_res.execution_time);
    
    // 5. Interaction (Call 'transfer')
    // We need to construct calldata for "transfer(address,uint256)"
    // Selector for "transfer" generic label in IR
    // In our Naive compiler, the dispatcher jumps based on... well, right now generic parsing just labels.
    // The previous mocked Quorlin compiler had specific logic to check specific selectors?
    // Let's look at the generated IR logic.
    // The Generic Quorlin Parser now generates a LABEL for "transfer".
    // But does it generate the JUMP table?
    // Looking at QuorlinFrontend: It pushes generic labels.
    // The IR currently DOES NOT auto-generate the limit check / jump table for labels.
    // The Codegen DOES NOT generate the Jump Table either.
    // So the bytecode is linear execution falling through labels?
    
    // Wait, IR `Label` is just a marker. The dispatched logic was manually pushed in the *old* implementation using `if source.contains...`.
    // My new generic implementation REMOVED the specific dispatch logic "if selector == ... jump".
    // It iterate lines and pushes `program.statements.push(IrStatement::Label(name))`.
    // But it DOES NOT generate the `EQ -> JUMPI` dispatch logic at the top!
    
    // Realization: Generic parsing is great, but without a generic DISPATCHER GENERATOR, the labels are unreachable or just fall-through.
    // I need to fix QuorlinFrontend to collect functions, THEN generate header dispatch logic.
    
    Ok(())
}
