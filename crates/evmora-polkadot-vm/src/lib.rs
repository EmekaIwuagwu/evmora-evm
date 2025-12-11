// Polkadot/Substrate VM - Simulates WASM Smart Contract execution
use anyhow::{Result, bail};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Balance(pub u128);

pub struct PolkadotVM {
    storage: HashMap<Vec<u8>, Vec<u8>>,
    balances: HashMap<[u8; 32], Balance>,
    debug_buffer: Vec<String>,
}

impl PolkadotVM {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            balances: HashMap::new(),
            debug_buffer: Vec::new(),
        }
    }

    pub fn set_balance(&mut self, account: [u8; 32], amount: u128) {
        self.balances.insert(account, Balance(amount));
    }

    pub fn get_balance(&self, account: &[u8; 32]) -> u128 {
        self.balances.get(account).map(|b| b.0).unwrap_or(0)
    }

    // Function to simulate contract calls based on selector and data
    pub fn execute_call(&mut self, _contract: [u8; 32], selector: [u8; 4], input: &[u8]) -> Result<Vec<u8>> {
        // In a real WASM VM, this would load the WASM blob and execute it.
        // Here we simulate the logic usually found in ink! contracts provided in the examples.
        
        // Selector for "transfer": 0xdeadbeef (example)
        // Selector for "balance_of": 0x12345678 (example)
        
        match selector {
            // transfer(to, amount)
            [0xde, 0xad, 0xbe, 0xef] => self.sim_transfer(input),
            // balance_of(account)
            [0x12, 0x34, 0x56, 0x78] => self.sim_balance_of(input),
            // constructor / init
            [0x00, 0x00, 0x00, 0x00] => self.sim_init(input),
            _ => bail!("Unknown function selector: {:02x?}", selector),
        }
    }

    fn sim_init(&mut self, _input: &[u8]) -> Result<Vec<u8>> {
        // Constructor logic
        self.debug_buffer.push("Contract initialized".to_string());
        Ok(vec![])
    }

    fn sim_transfer(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        // Expecting: [from: 32 bytes][to: 32 bytes][amount: 16 bytes (u128)]
        if input.len() < 80 {
            bail!("Invalid input for transfer");
        }
        
        let mut from = [0u8; 32];
        let mut to = [0u8; 32];
        let mut amount_bytes = [0u8; 16];
        
        from.copy_from_slice(&input[0..32]);
        to.copy_from_slice(&input[32..64]);
        amount_bytes.copy_from_slice(&input[64..80]);
        let amount = u128::from_le_bytes(amount_bytes);

        let from_bal = self.get_balance(&from);
        if from_bal < amount {
            bail!("Insufficient funds");
        }

        self.set_balance(from, from_bal - amount);
        let to_bal = self.get_balance(&to);
        self.set_balance(to, to_bal + amount);

        Ok(vec![])
    }

    fn sim_balance_of(&self, input: &[u8]) -> Result<Vec<u8>> {
        if input.len() < 32 {
            bail!("Invalid input for balance_of");
        }
        let mut addr = [0u8; 32];
        addr.copy_from_slice(&input[0..32]);
        
        let bal = self.get_balance(&addr);
        Ok(bal.to_le_bytes().to_vec())
    }
}

impl Default for PolkadotVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let mut vm = PolkadotVM::new();
        let alice = [1u8; 32];
        let bob = [2u8; 32];
        
        vm.set_balance(alice, 100);
        vm.set_balance(bob, 0);
        
        // Mock encode call: from(alice) + to(bob) + amount(40)
        let mut input = Vec::new();
        input.extend_from_slice(&alice);
        input.extend_from_slice(&bob);
        input.extend_from_slice(&40u128.to_le_bytes());
        
        // Call transfer
        vm.execute_call([0; 32], [0xde, 0xad, 0xbe, 0xef], &input).unwrap();
        
        assert_eq!(vm.get_balance(&alice), 60);
        assert_eq!(vm.get_balance(&bob), 40);
    }
}
