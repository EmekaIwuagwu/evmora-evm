use primitive_types::{H256, H160};
use std::collections::HashMap;
use evmora_plugins::{StorageBackend, EvmPlugin};
use anyhow::Result;

#[derive(Clone, Default)]
pub struct InMemoryStorage {
    data: HashMap<H160, HashMap<H256, H256>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl EvmPlugin for InMemoryStorage {
    fn name(&self) -> &str {
        "InMemoryStorage"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl StorageBackend for InMemoryStorage {
    fn get_storage(&self, address: H160, key: H256) -> Result<H256> {
        if let Some(account_storage) = self.data.get(&address) {
            Ok(*account_storage.get(&key).unwrap_or(&H256::zero()))
        } else {
            Ok(H256::zero())
        }
    }

    fn set_storage(&mut self, address: H160, key: H256, value: H256) -> Result<()> {
        let account_storage = self.data.entry(address).or_insert_with(HashMap::new);
        account_storage.insert(key, value);
        Ok(())
    }
}
