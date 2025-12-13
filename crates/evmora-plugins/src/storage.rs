use crate::traits::{EvmPlugin, StorageBackend};
use anyhow::Result;
use primitive_types::{H160, H256};
use std::collections::HashMap;
use parking_lot::RwLock;

/// In-memory storage backend for testing and development
#[derive(Debug)]
pub struct InMemoryStorage {
    storage: RwLock<HashMap<(H160, H256), H256>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
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
        self.storage.write().clear();
        Ok(())
    }
}

impl StorageBackend for InMemoryStorage {
    fn get_storage(&self, address: H160, key: H256) -> Result<H256> {
        let storage = self.storage.read();
        Ok(storage.get(&(address, key)).copied().unwrap_or(H256::zero()))
    }

    fn set_storage(&mut self, address: H160, key: H256, value: H256) -> Result<()> {
        let mut storage = self.storage.write();
        storage.insert((address, key), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_get_set() {
        let mut storage = InMemoryStorage::new();
        let addr = H160::from([1u8; 20]);
        let key = H256::from([2u8; 32]);
        let value = H256::from([3u8; 32]);

        // Initially should be zero
        assert_eq!(storage.get_storage(addr, key).unwrap(), H256::zero());

        // Set and get
        storage.set_storage(addr, key, value).unwrap();
        assert_eq!(storage.get_storage(addr, key).unwrap(), value);
    }

    #[test]
    fn test_storage_isolation() {
        let mut storage = InMemoryStorage::new();
        let addr1 = H160::from([1u8; 20]);
        let addr2 = H160::from([2u8; 20]);
        let key = H256::from([1u8; 32]);
        let value1 = H256::from([10u8; 32]);
        let value2 = H256::from([20u8; 32]);

        storage.set_storage(addr1, key, value1).unwrap();
        storage.set_storage(addr2, key, value2).unwrap();

        // Different addresses should have different values
        assert_eq!(storage.get_storage(addr1, key).unwrap(), value1);
        assert_eq!(storage.get_storage(addr2, key).unwrap(), value2);
    }
}
