use evmora_compiler::{Compiler, CompileOpts};
use evmora_runtime::{EvmClient, Transaction};
use std::path::{Path, PathBuf};
use std::env;

fn get_fixture_path(lang: &str, file: &str) -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&manifest_dir).join("../../tests/fixtures").join(lang).join(file)
}

fn wrap_in_init_code(runtime_code: &[u8]) -> Vec<u8> {
    let len = runtime_code.len();
    assert!(len < 65536, "Code too long for PUSH2 wrapper");
    let mut code = Vec::new();
    code.push(0x61);
    code.extend_from_slice(&(len as u16).to_be_bytes());
    code.push(0x80);
    code.push(0x60);
    code.push(0x0c);
    code.push(0x60);
    code.push(0x00);
    code.push(0x39);
    code.push(0x60);
    code.push(0x00);
    code.push(0xf3);
    code.extend_from_slice(runtime_code);
    code
}

#[tokio::test]
async fn test_multilang_e2e_counter() -> anyhow::Result<()> {
    let langs = vec![
        ("sol", "Counter.sol"),
        ("ql", "Counter.ql"),
        ("vy", "Counter.vy"),
        ("move", "Counter.move"),
    ];

    for (lang, file) in langs {
        println!("Testing {}...", lang);
        let path = get_fixture_path(lang, file);
        if !path.exists() {
             println!("Skipping {} - not found", lang);
             continue;
        }

        // 1. Compile
        let compiler = Compiler::new();
        let opts = CompileOpts { language: Some(lang.to_string()), deterministic: true };
        let artifact = compiler.compile_file(path.to_str().unwrap(), opts)?;
        
        let runtime_bytecode = hex::decode(&artifact.bytecode)?;
        let init_code = wrap_in_init_code(&runtime_bytecode);

        // 2. Deploy
        let mut client = EvmClient::new("test.toml")?;
        let deploy_tx = Transaction::create(
            init_code,
            vec![],
            1_000_000,
        );
        
        let result = client.execute(deploy_tx).await?;
        assert!(result.success, "Deployment failed for {}", lang);
        let contract_addr = result.contract_address.unwrap();
        println!("Deployed {} Counter at {:?}", lang, contract_addr);
        println!("Deployment gas used: {}", result.gas_used);

        // 3. Call increment
        let call_tx = Transaction::call(
            contract_addr,
            vec![0xd0, 0x9d, 0xe0, 0x8a], // increment() selector
            1_000_000,
        );
        
        let exec_res = client.execute(call_tx).await?;
        assert!(exec_res.success, "Execution failed for {}", lang);
        println!("Execution gas used: {}", exec_res.gas_used);
        
        let val = client.get_storage_at(contract_addr, primitive_types::H256::zero())?;
        assert_eq!(val, primitive_types::H256::from_low_u64_be(1), "Storage expected 1 for {}", lang);
        println!("✓ Test passed for {}\n", lang);
    }
    Ok(())
}
