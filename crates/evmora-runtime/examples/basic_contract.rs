use evmora_runtime::{EvmClient, Transaction};
use anyhow::Result;
use primitive_types::U256;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize EVM with config
    let mut client = EvmClient::new("evmora.toml")?;
    
    // Simple contract: PUSH1 42 PUSH1 0 MSTORE PUSH1 32 PUSH1 0 RETURN
    // 60 2a 60 00 52 60 20 60 00 f3
    let bytecode = hex::decode("602a60005260206000f3")?;
    
    // Deploy contract (simulate with CREATE ignored for now in my simplistic client, just execute bytecode)
    // My client currently executes tx.data as bytecode if it's a create check?
    // In executor.rs I just execute the code.
    
    let deploy_tx = Transaction::create(
        bytecode,
        vec![], 
        1_000_000, 
    );
    
    println!("Executing bytecode...");
    let result = client.execute(deploy_tx).await?;
    
    println!("Gas used: {}", result.gas_used);
    println!("Execution time: {:?}", result.execution_time);
    println!("Return data: {}", hex::encode(&result.return_data));
    
    // The return data should be 32 bytes of 42 (0x2a)
    assert_eq!(result.return_data.len(), 32);
    // U256::from(42) as bytes
    
    Ok(())
}
