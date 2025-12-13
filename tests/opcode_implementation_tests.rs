//! Comprehensive tests for memory gas metering and opcode implementations

#[cfg(test)]
mod memory_tests {
    use evmora_core::evm::Memory;
    use evmora_utils::EvmError;

    #[test]
    fn test_memory_gas_expansion_first_word() {
        let mut memory = Memory::new();
        let mut gas = 10000;
        
        // First expansion to 32 bytes (1 word) should cost 3 gas
        memory.resize_with_gas(0, 32, &mut gas).unwrap();
        assert_eq!(gas, 10000 - 3, "First word should cost 3 gas");
    }
    
    #[test]
    fn test_memory_gas_quadratic_growth() {
        let mut memory = Memory::new();
        let mut gas = 100000;
        
        // Track gas costs for increasing memory
        let initial_gas = gas;
        memory.resize_with_gas(0, 32, &mut gas).unwrap();
        let cost_1_word = initial_gas - gas;
        
        memory.resize_with_gas(0, 320, &mut gas).unwrap(); // 10 words
        let cost_10_words = initial_gas - cost_1_word - gas;
        
        memory.resize_with_gas(0, 3200, &mut gas).unwrap(); // 100 words
        let cost_100_words = initial_gas - cost_1_word - cost_10_words - gas;
        
        // Costs should increase (quadratic formula)
        assert!(cost_10_words > cost_1_word);
        assert!(cost_100_words > cost_10_words);
    }
    
    #[test]
    fn test_memory_dos_protection() {
        let mut memory = Memory::new();
        
        // Should reject allocation beyond 128 MB
        let result = memory.resize(0, 200 * 1024 * 1024);
        assert!(matches!(result, Err(EvmError::MemoryLimitExceeded)));
    }
    
    #[test]
    fn test_memory_out_of_gas() {
        let mut memory = Memory::new();
        let mut gas = 1; // Very low gas
        
        // Should fail with OutOfGas when trying to expand
        let result = memory.resize_with_gas(0, 1024, &mut gas);
        assert!(matches!(result, Err(EvmError::OutOfGas)));
    }
    
    #[test]
    fn test_memory_yellow_paper_formula() {
        let memory = Memory::new();
        
        // Verify Yellow Paper formula: cost = (words^2 / 512) + (3 * words)
        assert_eq!(memory.calculate_expansion_gas(32).unwrap(), 3);   // 1 word
        assert_eq!(memory.calculate_expansion_gas(64).unwrap(), 6);   // 2 words total
        assert_eq!(memory.calculate_expansion_gas(320).unwrap(), 30); // 10 words total
    }
    
    #[test]
    fn test_memory_operations_with_gas() {
        let mut memory = Memory::new();
        let mut gas = 10000;
        
        // Store with gas charging
        let data = vec![1, 2, 3, 4];
        memory.store_with_gas(0, &data, &mut gas).unwrap();
        
        // Verify data was stored
        let loaded = memory.load(0, 4).unwrap();
        assert_eq!(loaded, data);
        
        // Verify gas was charged
        assert!(gas < 10000);
    }
}

#[cfg(test)]
mod signed_arithmetic_tests {
    use evmora_core::evm::Stack;
    use evmora_core::evm::opcodes_extended::{op_sdiv, op_smod, op_slt, op_sgt};
    use primitive_types::U256;

    fn to_signed(value: i64) -> U256 {
        if value >= 0 {
            U256::from(value as u64)
        } else {
            let abs = (-value) as u64;
            (!U256::from(abs)).overflowing_add(U256::one()).0
        }
    }

    #[test]
    fn test_sdiv_positive_numbers() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(U256::from(3)).unwrap();
        op_sdiv(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(3));
    }
    
    #[test]
    fn test_sdiv_negative_dividend() {
        let mut stack = Stack::new();
        stack.push(to_signed(-10)).unwrap();
        stack.push(U256::from(3)).unwrap();
        op_sdiv(&mut stack).unwrap();
        
        let result = stack.pop().unwrap();
        assert_eq!(result, to_signed(-3));
    }
    
    #[test]
    fn test_sdiv_negative_divisor() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(to_signed(-3)).unwrap();
        op_sdiv(&mut stack).unwrap();
        
        let result = stack.pop().unwrap();
        assert_eq!(result, to_signed(-3));
    }
    
    #[test]
    fn test_sdiv_both_negative() {
        let mut stack = Stack::new();
        stack.push(to_signed(-10)).unwrap();
        stack.push(to_signed(-3)).unwrap();
        op_sdiv(&mut stack).unwrap();
        
        let result = stack.pop().unwrap();
        assert_eq!(result, U256::from(3));
    }
    
    #[test]
    fn test_sdiv_by_zero() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(U256::zero()).unwrap();
        op_sdiv(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::zero());
    }
    
    #[test]
    fn test_sdiv_overflow() {
        let mut stack = Stack::new();
        let min_value = U256::one() << 255; // -2^255
        let neg_one = U256::MAX;
        
        stack.push(min_value).unwrap();
        stack.push(neg_one).unwrap();
        op_sdiv(&mut stack).unwrap();
        
        // MIN / -1 should return MIN (not overflow)
        assert_eq!(stack.pop().unwrap(), min_value);
    }
    
    #[test]
    fn test_smod_positive() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(U256::from(3)).unwrap();
        op_smod(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
    }
    
    #[test]
    fn test_smod_negative_dividend() {
        let mut stack = Stack::new();
        stack.push(to_signed(-10)).unwrap();
        stack.push(U256::from(3)).unwrap();
        op_smod(&mut stack).unwrap();
        
        // Result takes sign of dividend
        let result = stack.pop().unwrap();
        assert_eq!(result, to_signed(-1));
    }
    
    #[test]
    fn test_slt_negative_vs_positive() {
        let mut stack = Stack::new();
        stack.push(to_signed(-5)).unwrap();
        stack.push(U256::from(10)).unwrap();
        op_slt(&mut stack).unwrap();
        
        // -5 < 10 should be true
        assert_eq!(stack.pop().unwrap(), U256::one());
    }
    
    #[test]
    fn test_slt_positive_vs_negative() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(to_signed(-5)).unwrap();
        op_slt(&mut stack).unwrap();
        
        // 10 < -5 should be false
        assert_eq!(stack.pop().unwrap(), U256::zero());
    }
    
    #[test]
    fn test_sgt_negative_vs_positive() {
        let mut stack = Stack::new();
        stack.push(to_signed(-5)).unwrap();
        stack.push(U256::from(10)).unwrap();
        op_sgt(&mut stack).unwrap();
        
        // -5 > 10 should be false
        assert_eq!(stack.pop().unwrap(), U256::zero());
    }
    
    #[test]
    fn test_sgt_positive_vs_negative() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(to_signed(-5)).unwrap();
        op_sgt(&mut stack).unwrap();
        
        // 10 > -5 should be true
        assert_eq!(stack.pop().unwrap(), U256::one());
    }
}

#[cfg(test)]
mod call_opcode_tests {
    use evmora_core::evm::{Stack, Memory};
    use evmora_core::evm::context::ExecutionContext;
    use evmora_core::evm::opcodes_extended::op_call;
    use primitive_types::{U256, H160};

    #[test]
    fn test_call_depth_limit() {
        let mut stack = Stack::new();
        let mut memory = Memory::new();
        let mut gas = 100000;
        let context = ExecutionContext::default();
        
        // Setup stack for CALL
        stack.push(U256::from(10000)).unwrap(); // gas
        stack.push(U256::from_big_endian(&[1u8; 32])).unwrap(); // address
        stack.push(U256::zero()).unwrap(); // value
        stack.push(U256::zero()).unwrap(); // argsOffset
        stack.push(U256::zero()).unwrap(); // argsSize
        stack.push(U256::zero()).unwrap(); // retOffset
        stack.push(U256::zero()).unwrap(); // retSize
        
        // Call at depth 1024 should fail
        let result = op_call(&mut stack, &mut memory, &mut gas, &context, 1024);
        assert_eq!(result.unwrap(), U256::zero()); // Failure
    }
    
    #[test]
    fn test_call_gas_deduction() {
        let mut stack = Stack::new();
        let mut memory = Memory::new();
        let mut gas = 100000;
        let context = ExecutionContext::default();
        
        // Setup stack
        stack.push(U256::from(10000)).unwrap();
        stack.push(U256::from_big_endian(&[1u8; 32])).unwrap();
        stack.push(U256::zero()).unwrap(); // no value transfer
        stack.push(U256::zero()).unwrap();
        stack.push(U256::zero()).unwrap();
        stack.push(U256::zero()).unwrap();
        stack.push(U256::zero()).unwrap();
        
        let initial_gas = gas;
        op_call(&mut stack, &mut memory, &mut gas, &context, 0).unwrap();
        
        // Should have deducted at least 700 gas (base cost)
        assert!(gas < initial_gas);
        assert!(initial_gas - gas >= 700);
    }
}
