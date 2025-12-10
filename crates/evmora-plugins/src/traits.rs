use async_trait::async_trait;
use anyhow::Result;
use primitive_types::{U256, H256, H160};

pub trait EvmPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
}

pub trait GasCalculator: EvmPlugin {
    fn calculate_gas(&self, opcode: u8, stack_depth: usize) -> u64;
}

#[async_trait]
pub trait BridgeAdapter: EvmPlugin {
    async fn transfer_token(&self, token: H160, to_chain: u64, amount: U256) -> Result<H256>;
    async fn receive_token(&self, proof: Vec<u8>) -> Result<()>;
}

pub trait StorageBackend: EvmPlugin {
    fn get_storage(&self, address: H160, key: H256) -> Result<H256>;
    fn set_storage(&mut self, address: H160, key: H256, value: H256) -> Result<()>;
}
