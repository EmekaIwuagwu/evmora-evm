use evmora_compiler::frontends::quorlin::QuorlinFrontend;
use evmora_compiler::frontends::traits::CompilerFrontend;
use evmora_compiler::codegen::Codegen;
use evmora_runtime::EvmClient;
use evmora_core::types::{Transaction, Address};
use primitive_types::U256;
use primitive_types::H256;

#[tokio::test]
async fn test_e2e_quorlin_contract_deployment_and_execution() {
    println!("Step 1: Compiling Quorlin Contract");
    let source = r#"
    contract Counter {
        uint256 count;

        fn increment() {
             count += 1;
        }
    }
    "#;
    
    let frontend = QuorlinFrontend;
    let ir_program = frontend.compile_to_ir(source, None).expect("Compilation failed");
    
    let codegen = Codegen::new();
    let bytecode = codegen.generate(&ir_program);
    println!("Compiled Bytecode Length: {} bytes", bytecode.len());
    
    println!("Step 2: Starting EVM Client");
    // Config path dummy
    let mut client = EvmClient::new("evmora.toml").expect("Failed to start client");
    
    println!("Step 3: Deploying Contract");
    let mut deploy_bytes = bytecode;
    // Note: In real EVM, deployment code returns runtime code.
    // Our Quorlin compiler generates runtime code directly (simple model).
    // So we wrap it in minimal init code: PUSH runtime_len, PUSH runtime_code, RETURN needed?
    // Wait, EvmClient logic for CREATE: "Execute Init Code... if success, store RETURN DATA as runtime code"
    // So if my bytecode *is* the runtime code, passing it as init code will execute it.
    // Executing runtime code usually reverts or stops. If it stops, return data is empty?
    // If I want to deploy 'compiled code', I need init code that returns it.
    
    // Minimal init code generator:
    // PUSH <len>
    // PUSH <offset> (where code starts)
    // PUSH 0 (mem dest)
    // CODECOPY
    // PUSH <len>
    // PUSH 0
    // RETURN
    // [CODE]
    
    // BUT, for this E2E, I can simplify:
    // EvmClient's CREATE logic executes 'data'.
    // If 'data' is just the runtime logic, it runs once and returns nothing?
    // Then deployed code is empty.
    
    // Fix: I need to wrap the bytecode in a deployer or manually insert into `client.code_storage` for testing "Call".
    // Or I construct init code.
    
    // Let's manually insert bytecode for "Deployment" simulation to avoid complexity of init code generation here.
    // However, the test says "Deploying Contract".
    // I will use a hack: construct a transaction that creates the address, then hack the client's storage? 
    // No, `client` fields are private.
    // So I MUST provide valid init code.
    
    // Simple init code wrapper:
    // 0x60 <len> 0x60 0x0C (offset) 0x60 0x00 0x39 (CODECOPY) 0x60 <len> 0x60 0x00 0xf3 (RETURN)
    // Offset 12 (0x0C) is approximate.
    // PUSH1 (2) len (1) PUSH1 (2) off (1) PUSH1 (2) dest (1) CODECOPY (1) -> 10 bytes?
    
    let len = deploy_bytes.len();
    let mut init_code = Vec::new();
    
    // PUSH2 len (assuming len < 65536)
    init_code.push(0x61); 
    init_code.extend_from_slice(&(len as u16).to_be_bytes());
    
    // PUSH2 offset (current len of init + codecopy logic length)
    // Logic: PUSH2 len, PUSH2 off, PUSH1 0, CODECOPY, PUSH2 len, PUSH1 0, RETURN
    // Size: 3 + 3 + 2 + 1 + 3 + 2 + 1 = 15 bytes.
    let offset = 15u16;
    init_code.push(0x61);
    init_code.extend_from_slice(&offset.to_be_bytes());
    
    // PUSH1 0
    init_code.push(0x60);
    init_code.push(0x00);
    
    // CODECOPY (0x39)
    init_code.push(0x39);
    
    // PUSH2 len
    init_code.push(0x61);
    init_code.extend_from_slice(&(len as u16).to_be_bytes());
    
    // PUSH1 0
    init_code.push(0x60);
    init_code.push(0x00);
    
    // RETURN (0xf3)
    init_code.push(0xf3);
    
    // Append actual code
    init_code.extend_from_slice(&deploy_bytes);
    
    let deploy_tx = Transaction {
        from: Address::from_low_u64_be(0x100),
        to: None, // Creation
        value: U256::zero(),
        data: init_code, // Init code wrapping runtime code
        gas_limit: 1000000,
        nonce: 0,
        signature: None,
    };
    
    let result = client.execute(deploy_tx).await.expect("Deployment failed");
    if !result.success {
        println!("Deployment Return Data: {:?}", result.return_data);
    }
    assert!(result.success, "Deployment execution failed");
    let contract_address = result.contract_address.expect("No contract address returned");
    println!("Contract deployed at: {:?}", contract_address);
    
    println!("Step 4: Executing 'increment' Transaction");
    // Selector for 'increment' is 0xd09de08a (mocked in compiler)
    let selector = hex::decode("d09de08a").unwrap();
    
    let call_tx = Transaction {
        from: Address::from_low_u64_be(0x100),
        to: Some(contract_address),
        value: U256::zero(),
        data: selector,
        gas_limit: 100000,
        nonce: 1, 
        signature: None,
    };
    
    let call_result = client.execute(call_tx).await.expect("Call failed");
    assert!(call_result.success, "Function call failed");
    println!("Function executed. Gas used: {}", call_result.gas_used);
    
    println!("Step 5: Verifying State");
    // Count is at slot 0
    let slot_key = H256::zero();
    let slot_value = client.get_storage_at(contract_address, slot_key).expect("Failed to read storage");
    
    println!("Storage Slot 0 Value: {:?}", slot_value);
    assert_eq!(slot_value, H256::from_low_u64_be(1), "Count should be incremented to 1");
    
    println!("E2E Test Passed Successfully!");
}
