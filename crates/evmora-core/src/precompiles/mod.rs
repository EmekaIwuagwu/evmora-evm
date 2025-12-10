pub mod ecrecover;
pub mod sha256;

use crate::types::Address;
use crate::evm::{Memory, Stack};
use anyhow::Result;

pub trait Precompile {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)>;
}

pub fn is_precompile(address: Address) -> bool {
    let bytes = address.as_bytes();
    bytes[0..19].iter().all(|&b| b == 0) && bytes[19] > 0 && bytes[19] <= 9
}
