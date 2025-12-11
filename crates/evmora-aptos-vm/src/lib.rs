use anyhow::{Result, bail};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MoveResource {
    pub type_: String,
    pub data: Vec<u8>,
}

pub struct Account {
    pub address: [u8; 32],
    pub sequence_number: u64,
    pub resources: HashMap<String, MoveResource>,
    pub modules: HashMap<String, Vec<u8>>, // Name -> Bytecode
    pub balance: u64, // Simulating 0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>
}

impl Account {
    pub fn new(address: [u8; 32]) -> Self {
        Self {
            address,
            sequence_number: 0,
            resources: HashMap::new(),
            modules: HashMap::new(),
            balance: 0,
        }
    }
}

pub struct AptosVM {
    accounts: HashMap<[u8; 32], Account>,
    // Simulator configuration
    pub gas_unit_price: u64,
}

impl AptosVM {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            gas_unit_price: 100,
        }
    }

    pub fn create_account(&mut self, addr: [u8; 32]) {
        self.accounts.entry(addr).or_insert_with(|| Account::new(addr));
    }

    pub fn mint(&mut self, addr: [u8; 32], amount: u64) -> Result<()> {
        let account = self.accounts.get_mut(&addr).ok_or_else(|| anyhow::anyhow!("Account not found"))?;
        account.balance += amount;
        Ok(())
    }

    pub fn get_balance(&self, addr: &[u8; 32]) -> u64 {
        self.accounts.get(addr).map(|a| a.balance).unwrap_or(0)
    }

    pub fn publish_module(&mut self, sender: [u8; 32], module_name: String, bytecode: Vec<u8>) -> Result<()> {
        let account = self.accounts.get_mut(&sender).ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;
        account.modules.insert(module_name, bytecode);
        account.sequence_number += 1;
        Ok(())
    }

    pub fn execute_entry_function(&mut self, sender: [u8; 32], module: &str, function: &str, args: &[Vec<u8>]) -> Result<()> {
        // In a real Move VM, this would load the module and execute bytecode.
        // Here we simulate common entry functions like "0x1::coin::transfer"
        
        let account_exists = self.accounts.contains_key(&sender);
        if !account_exists {
            bail!("Sender does not exist");
        }

        // Simulate gas deduction
        // (Simplified)

        match (module, function) {
            ("0x1::coin", "transfer") | ("coin", "transfer") => {
                self.sim_transfer(sender, args)
            },
            // Add more simulated functions here
            _ => {
                // If module exists in sender's account, pretend we executed it
                let account = self.accounts.get(&sender).unwrap();
                if account.modules.contains_key(module) {
                    println!("Executed {}::{} (Simulated)", module, function);
                    Ok(())
                } else {
                    // Also check if it's a "standard library" module we implicitly support
                     println!("Executed {}::{} (Simulated Standard Lib)", module, function);
                     Ok(())
                }
            }
        }
    }

    fn sim_transfer(&mut self, sender: [u8; 32], args: &[Vec<u8>]) -> Result<()> {
        if args.len() < 2 {
            bail!("Invalid arguments for coin::transfer");
        }
        
        // Arg 0: Recipient Address (32 bytes)
        // Arg 1: Amount (u64)
        
        let recipient_bytes = &args[0];
        let amount_bytes = &args[1];
        
        if recipient_bytes.len() != 32 {
            bail!("Invalid recipient address length");
        }
        
        let mut recipient_addr = [0u8; 32];
        recipient_addr.copy_from_slice(recipient_bytes);
        
        // Basic u64 deserialization (little endian)
        if amount_bytes.len() > 8 {
             bail!("Invalid amount length");
        }
        let mut amt_arr = [0u8; 8];
        for (i, b) in amount_bytes.iter().enumerate() {
            if i < 8 { amt_arr[i] = *b; }
        }
        let amount = u64::from_le_bytes(amt_arr);

        // Perform transfer
        let sender_bal = self.get_balance(&sender);
        if sender_bal < amount {
            bail!("Insufficient balance");
        }
        
        // Deduct
        {
            let s_acc = self.accounts.get_mut(&sender).unwrap();
            s_acc.balance -= amount;
            s_acc.sequence_number += 1;
        }

        // Deposit
        if let Some(r_acc) = self.accounts.get_mut(&recipient_addr) {
            r_acc.balance += amount;
        } else {
            // Create account if not exists? Aptos requires creation usually, but let's be lenient
            let mut new_acc = Account::new(recipient_addr);
            new_acc.balance = amount;
            self.accounts.insert(recipient_addr, new_acc);
        }

        Ok(())
    }
}

impl Default for AptosVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aptos_transfer() {
        let mut vm = AptosVM::new();
        let alice = [1u8; 32];
        let bob = [2u8; 32];

        vm.create_account(alice);
        vm.mint(alice, 1000).unwrap();

        let amount = 500u64;
        let args = vec![
            bob.to_vec(),
            amount.to_le_bytes().to_vec()
        ];

        vm.execute_entry_function(alice, "0x1::coin", "transfer", &args).unwrap();

        assert_eq!(vm.get_balance(&alice), 500);
        assert_eq!(vm.get_balance(&bob), 500);
    }
}
