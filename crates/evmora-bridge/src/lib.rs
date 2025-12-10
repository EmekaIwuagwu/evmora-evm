use evmora_core::types::Address;
use primitive_types::{U256, H256};
use anyhow::Result;
use std::collections::HashMap;

pub use adapters::ChainAdapter;
pub use tokens::erc20::Erc20Token;

pub mod adapters;
pub mod tokens;
pub mod proof;

#[derive(Debug, Clone)]
pub struct BridgeTransaction {
    pub id: H256,
    pub token: Address,
    pub amount: U256,
    pub to_chain: u64,
    pub recipient: Address,
    pub lock_hash: String,
}

pub struct BridgeManager {
    adapters: HashMap<u64, Box<dyn ChainAdapter + Send + Sync>>,
}

impl BridgeManager {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    pub fn register_adapter(&mut self, chain_id: u64, adapter: Box<dyn ChainAdapter + Send + Sync>) {
        self.adapters.insert(chain_id, adapter);
    }

    pub async fn lock_and_bridge(
        &self,
        token: Address,
        amount: U256,
        to_chain: u64,
        recipient: Address,
    ) -> Result<BridgeTransaction> {
        let adapter = self.adapters.get(&to_chain)
            .ok_or_else(|| anyhow::anyhow!("Chain ID {} not supported", to_chain))?;

        // Lock tokens
        let lock_hash = adapter.lock(token, amount).await?;

        // Create bridge transaction record
        Ok(BridgeTransaction {
            id: H256::random(),
            token,
            amount,
            to_chain,
            recipient,
            lock_hash,
        })
    }
    
    pub async fn track_transaction(&self, _tx_id: H256) -> Result<String> {
        // Mock status
        Ok("Confirmed { confirmations: 12/12, relay_submitted: true }".to_string())
    }
}
