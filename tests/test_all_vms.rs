// Comprehensive VM tests for all supported chains

#[cfg(test)]
mod tests {
    use evmora_core::evm::{Executor, ExecutionContext};
    use evmora_core::types::Address;
    use evmora_plugins::{InMemoryStorage, StandardGasCalculator};
    use primitive_types::U256;
    
    #[test]
    fn test_evm_solidity_compatibility() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        
        let context = ExecutionContext {
            caller: Address::from_low_u64_be(1),
            origin: Address::from_low_u64_be(1),
            address: Address::from_low_u64_be(2),
            value: U256::from(100),
            data: vec![],
            gas_limit: 1_000_000,
            gas_price: U256::from(1),
            block_number: U256::from(1),
            block_timestamp: U256::from(1000),
            timestamp: 1000,
            chain_id: U256::from(1),
            coinbase: Address::zero(),
            difficulty: U256::zero(),
            base_fee: 0,
        };
        
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test basic arithmetic opcodes
        let code = vec![
            0x60, 0x05, // PUSH1 5
            0x60, 0x03, // PUSH1 3
            0x01,       // ADD
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xf3,       // RETURN
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
        assert_eq!(result.return_data.len(), 32);
    }
    
    #[test]
    fn test_evm_environmental_opcodes() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        
        let caller_addr = Address::from_low_u64_be(0x1234);
        let contract_addr = Address::from_low_u64_be(0x5678);
        
        let context = ExecutionContext {
            caller: caller_addr,
            origin: caller_addr,
            address: contract_addr,
            value: U256::from(1000),
            data: vec![0xaa, 0xbb, 0xcc, 0xdd],
            gas_limit: 1_000_000,
            gas_price: U256::from(10),
            block_number: U256::from(12345),
            block_timestamp: U256::from(1638360000),
            timestamp: 1638360000,
            chain_id: U256::from(1),
            coinbase: Address::from_low_u64_be(0x9999),
            difficulty: U256::from(5000000),
            base_fee: 7,
        };
        
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test CALLER, CALLVALUE, ADDRESS, TIMESTAMP, NUMBER, CHAINID
        let code = vec![
            0x33, // CALLER
            0x34, // CALLVALUE
            0x30, // ADDRESS
            0x42, // TIMESTAMP
            0x43, // NUMBER
            0x46, // CHAINID
            0x00, // STOP
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
    }
    
    #[test]
    fn test_evm_storage_operations() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        
        let context = ExecutionContext::default();
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test SSTORE and SLOAD
        let code = vec![
            0x60, 0x42, // PUSH1 0x42 (value)
            0x60, 0x01, // PUSH1 0x01 (key)
            0x55,       // SSTORE
            0x60, 0x01, // PUSH1 0x01 (key)
            0x54,       // SLOAD
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xf3,       // RETURN
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
        // Verify the loaded value is 0x42
        assert_eq!(result.return_data[31], 0x42);
    }
    
    #[test]
    fn test_quorlin_vm() {
        use evmora_quorlin_vm::QuorlinVM;
        
        let mut vm = QuorlinVM::new();
        
        // Test simple addition: PUSH 5, PUSH 3, ADD, HALT
        let code = vec![
            0x00, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // PUSH 5
            0x00, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // PUSH 3
            0x10, // ADD
            0xFF, // HALT
        ];
        
        let result = vm.execute(&code, [0; 20], 0).unwrap();
        assert_eq!(result.len(), 1);
    }
    
    #[test]
    fn test_solana_vm() {
        use evmora_solana_vm::SolanaVM;
        
        let mut vm = SolanaVM::new();
        let alice = [1u8; 32];
        let bob = [2u8; 32];
        
        // Create accounts
        vm.create_account(alice, 1000, 64);
        vm.create_account(bob, 1000, 64);
        
        // Initialize alice with 100
        let init_data = [0u8, 100, 0, 0, 0, 0, 0, 0, 0];
        vm.execute_instruction(&[0; 32], &[alice], &init_data).unwrap();
        
        // Transfer 30 from alice to bob
        let transfer_data = [1u8, 30, 0, 0, 0, 0, 0, 0, 0];
        vm.execute_instruction(&[0; 32], &[alice, bob], &transfer_data).unwrap();
        
        assert_eq!(vm.get_balance(&alice), 70);
        assert_eq!(vm.get_balance(&bob), 30);
    }
    
    #[test]
    fn test_polkadot_vm() {
        use evmora_polkadot_vm::PolkadotVM;
        
        let mut vm = PolkadotVM::new();
        let alice = [1u8; 32];
        let bob = [2u8; 32];
        
        vm.set_balance(alice, 100);
        vm.set_balance(bob, 0);
        
        // Mock transfer call: from(alice) + to(bob) + amount(40)
        let mut input = Vec::new();
        input.extend_from_slice(&alice);
        input.extend_from_slice(&bob);
        input.extend_from_slice(&40u128.to_le_bytes());
        
        // Call transfer
        vm.execute_call([0; 32], [0xde, 0xad, 0xbe, 0xef], &input).unwrap();
        
        assert_eq!(vm.get_balance(&alice), 60);
        assert_eq!(vm.get_balance(&bob), 40);
    }
    
    #[test]
    fn test_aptos_vm() {
        use evmora_aptos_vm::AptosVM;
        
        let mut vm = AptosVM::new();
        let alice = [1u8; 32];
        let bob = [2u8; 32];
        
        vm.create_account(alice);
        vm.mint(alice, 1000).unwrap();
        
        let amount = 500u64;
        let args = vec![
            bob.to_vec(),
            amount.to_le_bytes().to_vec()
        ];
        
        vm.execute_entry_function(alice, "0x1::coin", "transfer", &args).unwrap();
        
        assert_eq!(vm.get_balance(&alice), 500);
        assert_eq!(vm.get_balance(&bob), 500);
    }
    
    #[test]
    fn test_evm_bitwise_operations() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        let context = ExecutionContext::default();
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test AND, OR, XOR, NOT, SHL, SHR
        let code = vec![
            0x60, 0x0F, // PUSH1 15
            0x60, 0x03, // PUSH1 3
            0x16,       // AND (result: 3)
            0x60, 0x05, // PUSH1 5
            0x17,       // OR (result: 7)
            0x60, 0x01, // PUSH1 1
            0x1b,       // SHL (result: 14)
            0x00,       // STOP
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
    }
    
    #[test]
    fn test_evm_comparison_operations() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        let context = ExecutionContext::default();
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test LT, GT, EQ, ISZERO
        let code = vec![
            0x60, 0x05, // PUSH1 5
            0x60, 0x03, // PUSH1 3
            0x10,       // LT (result: 0, 3 < 5 is false in stack order)
            0x60, 0x05, // PUSH1 5
            0x60, 0x05, // PUSH1 5
            0x14,       // EQ (result: 1)
            0x15,       // ISZERO (result: 0)
            0x00,       // STOP
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
    }
    
    #[test]
    fn test_evm_jump_operations() {
        let mut storage = InMemoryStorage::new();
        let gas_calc = Box::new(StandardGasCalculator);
        let context = ExecutionContext::default();
        let mut executor = Executor::new(context, &mut storage, gas_calc);
        
        // Test JUMP and JUMPI
        let code = vec![
            0x60, 0x08, // PUSH1 8 (jump destination)
            0x56,       // JUMP
            0x60, 0xFF, // PUSH1 255 (should be skipped)
            0x00,       // STOP (should be skipped)
            0x5b,       // JUMPDEST (offset 8)
            0x60, 0x42, // PUSH1 66
            0x00,       // STOP
        ];
        
        let result = executor.execute(&code).unwrap();
        assert!(result.success);
    }
}
