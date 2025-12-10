use primitive_types::U256;
use evmora_core::types::Address;

pub struct Erc721Token {
    pub address: Address,
    pub name: String,
    pub symbol: String,
}

impl Erc721Token {
    pub fn new(address: Address, name: String, symbol: String) -> Self {
        Self { address, name, symbol }
    }
}
