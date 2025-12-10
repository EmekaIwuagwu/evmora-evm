use super::ChainAdapter;
use async_trait::async_trait;
use evmora_core::types::Address;
use crate::Result;
use primitive_types::U256;

pub struct PolygonAdapter;

#[async_trait]
impl ChainAdapter for PolygonAdapter {
    async fn lock(&self, _token: Address, _amount: U256) -> Result<String> {
        Ok("0x123...poly".to_string())
    }
}
