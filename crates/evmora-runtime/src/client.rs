use evmora_core::{Executor, ExecutionContext, Transaction, ExecutionResult};
use evmora_core::evm::storage::InMemoryStorage;
use evmora_core::gas::StandardGasCalculator;
use evmora_utils::EvmConfig;
use evmora_core::types::Address;
use anyhow::Result;

pub struct EvmClient {
    config: EvmConfig,
    storage: InMemoryStorage, 
}

impl EvmClient {
    pub fn new(_config_path: &str) -> Result<Self> {
        // Load config (stub)
        let config = EvmConfig::default(); 
        Ok(Self {
            config,
            storage: InMemoryStorage::new(),
        })
    }

    pub async fn execute(&mut self, transaction: Transaction) -> Result<ExecutionResult> {
        let mut context = ExecutionContext::default();
        context.gas_limit = transaction.gas_limit;
        context.value = transaction.value;
        context.data = transaction.data.clone();
        context.caller = transaction.from;
        
        // 1. Calculate Intrinsic Gas
        // Base cost: 21000
        // Data cost: 4 gas per zero byte, 16 gas per non-zero byte
        let mut intrinsic_gas = 21000;
        for byte in &transaction.data {
             if *byte == 0 {
                 intrinsic_gas += 4;
             } else {
                 intrinsic_gas += 16;
             }
        }
        
        // Contract creation cost (init code execution + storage)
        if transaction.to.is_none() {
             intrinsic_gas += 32000; // Init code base cost
        }

        if transaction.gas_limit < intrinsic_gas {
             return Ok(
                 ExecutionResult {
                     success: false,
                     return_data: vec![],
                     gas_used: transaction.gas_limit,
                     contract_address: None,
                     execution_time: std::time::Duration::from_secs(0),
                 }
             );
        }
        
        // Deduct intrinsic gas before execution starts
        let remaining_gas = transaction.gas_limit - intrinsic_gas;
        context.gas_limit = remaining_gas;

        if let Some(to) = transaction.to {
            context.address = to;
        } else {
            // Contract creation: generate address
            // Mock: simplistic address gen
            let mut bytes = [0u8; 20];
            bytes[0] = 0xff; // distinguishing mark
            context.address = Address::from_slice(&bytes); 
        }

        let gas_calculator = Box::new(StandardGasCalculator);
        
        // Use mutable reference to self.storage which implements StorageBackend
        let mut executor = Executor::new(context, &mut self.storage, gas_calculator);
        
        // Execute
        let mut res = executor.execute(&transaction.data)?;
        
        // Add back intrinsic gas to total used
        res.gas_used += intrinsic_gas;
        
        Ok(res)
    }
}
