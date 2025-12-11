// Solana BPF VM - Simplified Berkeley Packet Filter interpreter
use anyhow::{Result, bail};
use std::collections::HashMap;

pub struct Account {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}

impl Account {
    pub fn new(lamports: u64, size: usize) -> Self {
        Self {
            lamports,
            data: vec![0; size],
            owner: [0; 32],
            is_signer: false,
            is_writable: true,
        }
    }
}

pub struct SolanaVM {
    accounts: HashMap<[u8; 32], Account>,
    registers: [u64; 11], // r0-r10
    memory: Vec<u8>,
    pc: usize,
}

impl SolanaVM {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            registers: [0; 11],
            memory: vec![0; 65536],
            pc: 0,
        }
    }

    pub fn create_account(&mut self, pubkey: [u8; 32], lamports: u64, size: usize) {
        self.accounts.insert(pubkey, Account::new(lamports, size));
    }

    pub fn execute_instruction(&mut self, program_id: &[u8; 32], accounts: &[[u8; 32]], data: &[u8]) -> Result<()> {
        // Simplified instruction execution
        // In real Solana, this would be BPF bytecode
        
        if data.is_empty() {
            return Ok(());
        }

        let instruction = data[0];
        
        match instruction {
            0 => self.initialize(accounts, data)?,
            1 => self.transfer(accounts, data)?,
            2 => self.read_balance(accounts)?,
            _ => bail!("Unknown instruction: {}", instruction),
        }

        Ok(())
    }

    fn initialize(&mut self, accounts: &[[u8; 32]], data: &[u8]) -> Result<()> {
        if accounts.is_empty() {
            bail!("No accounts provided");
        }

        let account_key = accounts[0];
        if let Some(account) = self.accounts.get_mut(&account_key) {
            if data.len() >= 9 {
                let initial_value = u64::from_le_bytes(data[1..9].try_into().unwrap());
                account.data[0..8].copy_from_slice(&initial_value.to_le_bytes());
            }
        }

        Ok(())
    }

    fn transfer(&mut self, accounts: &[[u8; 32]], data: &[u8]) -> Result<()> {
        if accounts.len() < 2 {
            bail!("Transfer requires 2 accounts");
        }

        if data.len() < 9 {
            bail!("Transfer requires amount data");
        }

        let from_key = accounts[0];
        let to_key = accounts[1];
        let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());

        // Read from balance
        let from_balance = if let Some(from_account) = self.accounts.get(&from_key) {
            u64::from_le_bytes(from_account.data[0..8].try_into().unwrap())
        } else {
            bail!("From account not found");
        };

        if from_balance < amount {
            bail!("Insufficient balance");
        }

        // Read to balance
        let to_balance = if let Some(to_account) = self.accounts.get(&to_key) {
            u64::from_le_bytes(to_account.data[0..8].try_into().unwrap())
        } else {
            0
        };

        // Update balances
        if let Some(from_account) = self.accounts.get_mut(&from_key) {
            from_account.data[0..8].copy_from_slice(&(from_balance - amount).to_le_bytes());
        }

        if let Some(to_account) = self.accounts.get_mut(&to_key) {
            to_account.data[0..8].copy_from_slice(&(to_balance + amount).to_le_bytes());
        }

        Ok(())
    }

    fn read_balance(&mut self, accounts: &[[u8; 32]]) -> Result<()> {
        if accounts.is_empty() {
            bail!("No account provided");
        }

        let account_key = accounts[0];
        if let Some(account) = self.accounts.get(&account_key) {
            let balance = u64::from_le_bytes(account.data[0..8].try_into().unwrap());
            self.registers[0] = balance; // Store in r0 (return value)
        }

        Ok(())
    }

    pub fn get_account(&self, pubkey: &[u8; 32]) -> Option<&Account> {
        self.accounts.get(pubkey)
    }

    pub fn get_balance(&self, pubkey: &[u8; 32]) -> u64 {
        self.accounts.get(pubkey)
            .and_then(|acc| acc.data.get(0..8))
            .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()))
            .unwrap_or(0)
    }
}

impl Default for SolanaVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_initialize() {
        let mut vm = SolanaVM::new();
        let pubkey = [1u8; 32];
        
        vm.create_account(pubkey, 1000, 64);
        
        // Initialize with value 100
        let data = [0u8, 100, 0, 0, 0, 0, 0, 0, 0];
        vm.execute_instruction(&[0; 32], &[pubkey], &data).unwrap();
        
        assert_eq!(vm.get_balance(&pubkey), 100);
    }

    #[test]
    fn test_transfer() {
        let mut vm = SolanaVM::new();
        let from = [1u8; 32];
        let to = [2u8; 32];
        
        vm.create_account(from, 1000, 64);
        vm.create_account(to, 1000, 64);
        
        // Initialize from with 100
        vm.execute_instruction(&[0; 32], &[from], &[0, 100, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        
        // Transfer 30 from -> to
        vm.execute_instruction(&[0; 32], &[from, to], &[1, 30, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        
        assert_eq!(vm.get_balance(&from), 70);
        assert_eq!(vm.get_balance(&to), 30);
    }
}
