use evmora_runtime::{EvmClient, Transaction};
use anyhow::Result;

#[tokio::test]
async fn test_simple_addition() -> Result<()> {
    let mut client = EvmClient::new("evmora.toml")?;
    
    // PUSH1 10 PUSH1 20 ADD PUSH1 0 MSTORE PUSH1 32 PUSH1 0 RETURN
    // 60 0a 60 14 01 60 00 52 60 20 60 00 f3
    let bytecode = hex::decode("600a60140160005260206000f3")?;
    
    let tx = Transaction::create(bytecode, vec![], 100000);
    let result = client.execute(tx).await?;
    
    assert!(result.success);
    // 10 + 20 = 30 (0x1e)
    assert_eq!(result.return_data[31], 0x1e);
    Ok(())
}
