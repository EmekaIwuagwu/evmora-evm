use primitive_types::U256;
use evmora_core::types::Address;

pub struct Erc20Token {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}
