use crate::types::Address;
use primitive_types::U256;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub caller: Address,
    pub origin: Address,
    pub address: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub block_number: U256,
    pub block_timestamp: U256,
    pub timestamp: u64,
    pub chain_id: U256,
    pub coinbase: Address,
    pub difficulty: U256,
    pub base_fee: u64,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            caller: Address::zero(),
            origin: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1_000_000,
            gas_price: U256::zero(),
            block_number: U256::zero(),
            block_timestamp: U256::zero(),
            timestamp: 0,
            chain_id: U256::one(),
            coinbase: Address::zero(),
            difficulty: U256::zero(),
            base_fee: 0,
        }
    }
}
