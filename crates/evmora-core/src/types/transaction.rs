use primitive_types::{U256, H256};
use crate::types::Address;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: u64,
    pub from: Address, // Mock sender for now, in read tx this is recovered from v,r,s
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

impl Transaction {
    pub fn create(data: Vec<u8>, mut args: Vec<u8>, gas_limit: u64) -> Self {
        let mut full_data = data;
        full_data.append(&mut args);
        Self {
            nonce: U256::zero(),
            gas_price: U256::zero(),
            gas_limit,
            from: Address::zero(), // Default sender
            to: None,
            value: U256::zero(),
            data: full_data,
            v: 0,
            r: H256::zero(),
            s: H256::zero(),
        }
    }
    
    pub fn call(to: Address, data: Vec<u8>, gas_limit: u64) -> Self {
         Self {
            nonce: U256::zero(),
            gas_price: U256::zero(),
            gas_limit,
            from: Address::zero(), // Default sender
            to: Some(to),
            value: U256::zero(),
            data,
            v: 0,
            r: H256::zero(),
            s: H256::zero(),
        }
    }
}
