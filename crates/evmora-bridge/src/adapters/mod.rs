pub mod ethereum;
pub mod polygon;

use async_trait::async_trait;
use crate::Result;
use evmora_core::types::Address;
use primitive_types::U256;

#[async_trait]
pub trait ChainAdapter {
    async fn lock(&self, token: Address, amount: U256) -> Result<String>; // Returns tx hash
}
