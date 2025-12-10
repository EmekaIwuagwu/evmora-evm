use evmora_core::{Executor, ExecutionContext, Transaction, ExecutionResult};
use evmora_core::evm::storage::InMemoryStorage;
use evmora_core::gas::StandardGasCalculator;
use evmora_utils::EvmConfig;
use evmora_core::types::Address;
use anyhow::Result;

use std::collections::HashMap;

pub struct EvmClient {
    pub config: EvmConfig,
    storage: InMemoryStorage,
    code_storage: HashMap<Address, Vec<u8>>,
    nonce_counter: u64,
}

impl EvmClient {
    pub fn new(_config_path: &str) -> Result<Self> {
        // Load config (stub)
        let config = EvmConfig::default(); 
        Ok(Self {
            config,
            storage: InMemoryStorage::new(),
            code_storage: HashMap::new(),
            nonce_counter: 0,
        })
    }

    pub async fn execute(&mut self, transaction: Transaction) -> Result<ExecutionResult> {
        let mut context = ExecutionContext::default();
        context.gas_limit = transaction.gas_limit;
        context.value = transaction.value;
        context.data = transaction.data.clone();
        context.caller = transaction.from;
        
        // 1. Calculate Intrinsic Gas
        let mut intrinsic_gas = 21000;
        for byte in &transaction.data {
             if *byte == 0 {
                 intrinsic_gas += 4;
             } else {
                 intrinsic_gas += 16;
             }
        }
        
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
        
        let remaining_gas = transaction.gas_limit - intrinsic_gas;
        context.gas_limit = remaining_gas;

        let gas_calculator = Box::new(StandardGasCalculator);
        let mut executor = Executor::new(context.clone(), &mut self.storage, gas_calculator);

        if let Some(to) = transaction.to {
            // CALL
            context.address = to;
            executor.context.address = to; // Ensure executor knows context address
            
            let code = self.code_storage.get(&to).cloned().unwrap_or_default();
            
            // Execute code with context.data as calldata (already set in executor.context)
            let mut res = executor.execute(&code)?;
            
            res.gas_used += intrinsic_gas;
            Ok(res)
        } else {
            // CREATE
            // Generate address
            self.nonce_counter += 1;
            let mut bytes = [0u8; 20];
            // Simple deterministic address generation for testing
            let nonce_bytes = self.nonce_counter.to_be_bytes();
            bytes[12..20].copy_from_slice(&nonce_bytes);
            let address = Address::from_slice(&bytes);
            
            executor.context.address = address;
            
            // Execute Init Code (transaction.data)
            let mut res = executor.execute(&transaction.data)?;
            
            if res.success {
                // Store the RETURNED DATA as the runtime code
                self.code_storage.insert(address, res.return_data.clone());
                res.contract_address = Some(address);
            }
        
            res.gas_used += intrinsic_gas;
            Ok(res)
        }
    }
    
    pub fn get_storage_at(&self, address: Address, key: primitive_types::H256) -> Result<primitive_types::H256> {
         use evmora_plugins::StorageBackend;
         self.storage.get_storage(address, key)
    }
}
