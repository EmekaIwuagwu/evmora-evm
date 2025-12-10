use super::Precompile;
use anyhow::Result;
use sha3::{Digest, Sha3_256};

pub struct Sha256Precompile;

impl Precompile for Sha256Precompile {
    fn execute(&self, input: &[u8], _gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let mut hasher = Sha3_256::new();
        hasher.update(input);
        Ok((hasher.finalize().to_vec(), 60 + 12 * ((input.len() as u64 + 31) / 32)))
    }
}
