use evmora_utils::EvmError;

/// EVM Memory with proper gas metering
#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
    size_in_words: usize, // Track for gas calculation
}

impl Memory {
    pub fn new() -> Self {
        Self { 
            data: Vec::new(),
            size_in_words: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Calculate gas cost for memory expansion using Yellow Paper formula
    /// Formula: memory_cost = (size_in_words^2 / 512) + (3 * size_in_words)
    pub fn calculate_expansion_gas(&self, new_size_bytes: usize) -> Result<u64, EvmError> {
        let new_size_words = (new_size_bytes + 31) / 32;
        
        if new_size_words <= self.size_in_words {
            return Ok(0); // No expansion needed
        }
        
        let old_cost = self.memory_gas_cost(self.size_in_words);
        let new_cost = self.memory_gas_cost(new_size_words);
        
        Ok(new_cost.saturating_sub(old_cost))
    }
    
    /// Yellow Paper Appendix H: Memory cost formula
    fn memory_gas_cost(&self, size_in_words: usize) -> u64 {
        let size = size_in_words as u64;
        (size * size) / 512 + (3 * size)
    }

    /// Resize memory with gas tracking and DoS protection
    pub fn resize(&mut self, offset: usize, size: usize) -> Result<(), EvmError> {
        if size == 0 {
            return Ok(());
        }
        
        let end = offset.checked_add(size).ok_or(EvmError::MemoryViolation)?;
        
        // Hard limit to prevent DoS (128 MB)
        const MAX_MEMORY: usize = 128 * 1024 * 1024;
        if end > MAX_MEMORY {
            return Err(EvmError::MemoryLimitExceeded);
        }
        
        if end > self.data.len() {
            // Round up to multiple of 32
            let new_size = (end + 31) / 32 * 32;
            self.data.resize(new_size, 0);
            self.size_in_words = new_size / 32;
        }
        Ok(())
    }
    
    /// Resize with explicit gas charging
    pub fn resize_with_gas(&mut self, offset: usize, size: usize, gas: &mut u64) -> Result<(), EvmError> {
        if size == 0 {
            return Ok(());
        }
        
        let end = offset.checked_add(size).ok_or(EvmError::MemoryViolation)?;
        
        // Calculate gas before expansion
        let expansion_gas = self.calculate_expansion_gas(end)?;
        
        if *gas < expansion_gas {
            return Err(EvmError::OutOfGas);
        }
        
        *gas = gas.saturating_sub(expansion_gas);
        
        // Now actually resize
        self.resize(offset, size)?;
        
        Ok(())
    }

    pub fn load(&self, offset: usize, size: usize) -> Result<Vec<u8>, EvmError> {
        if offset + size > self.data.len() {
             // In EVM, reading past end returns 0s
             let mut res = Vec::with_capacity(size);
             if offset < self.data.len() {
                 let available = std::cmp::min(size, self.data.len() - offset);
                 res.extend_from_slice(&self.data[offset..offset+available]);
                 res.resize(size, 0);
             } else {
                 res.resize(size, 0);
             }
             return Ok(res);
        }
        Ok(self.data[offset..offset + size].to_vec())
    }
    
    pub fn load32(&self, offset: usize) -> Result<[u8; 32], EvmError> {
        if offset + 32 > self.data.len() {
            let mut res = [0u8; 32];
            for i in 0..32 {
                if offset + i < self.data.len() {
                    res[i] = self.data[offset + i];
                }
            }
            Ok(res)
        } else {
            let mut res = [0u8; 32];
            res.copy_from_slice(&self.data[offset..offset+32]);
            Ok(res)
        }
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) -> Result<(), EvmError> {
        self.resize(offset, value.len())?;
        self.data[offset..offset + value.len()].copy_from_slice(value);
        Ok(())
    }
    
    pub fn store_with_gas(&mut self, offset: usize, value: &[u8], gas: &mut u64) -> Result<(), EvmError> {
        self.resize_with_gas(offset, value.len(), gas)?;
        self.data[offset..offset + value.len()].copy_from_slice(value);
        Ok(())
    }
    
    pub fn store8(&mut self, offset: usize, value: u8) -> Result<(), EvmError> {
        self.resize(offset, 1)?;
        self.data[offset] = value;
        Ok(())
    }
    
    pub fn get_slice(&self, offset: usize, size: usize) -> Result<&[u8], EvmError> {
        if offset + size > self.data.len() {
            return Err(EvmError::MemoryViolation);
        }
        Ok(&self.data[offset..offset + size])
    }
    
    pub fn set_slice(&mut self, offset: usize, data: &[u8]) -> Result<(), EvmError> {
        self.resize(offset, data.len())?;
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_gas_expansion() {
        let mut memory = Memory::new();
        let mut gas = 10000;
        
        // First expansion should cost gas
        memory.resize_with_gas(0, 32, &mut gas).unwrap();
        assert!(gas < 10000, "First expansion should cost gas");
        
        let gas_after_first = gas;
        
        // Second expansion should cost more
        memory.resize_with_gas(0, 1024, &mut gas).unwrap();
        assert!(gas < gas_after_first, "Second expansion should cost more gas");
    }
    
    #[test]
    fn test_memory_dos_prevention() {
        let mut memory = Memory::new();
        
        // Should reject excessive memory allocation
        let result = memory.resize(0, 200 * 1024 * 1024);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_yellow_paper_gas_formula() {
        let memory = Memory::new();
        
        // For 1 word (32 bytes): cost = (1^2)/512 + 3*1 = 3
        assert_eq!(memory.memory_gas_cost(1), 3);
        
        // For 10 words: cost = (10^2)/512 + 3*10 = 30
        assert_eq!(memory.memory_gas_cost(10), 30);
        
        // For 100 words: cost = (100^2)/512 + 3*100 = 319
        assert_eq!(memory.memory_gas_cost(100), 319);
    }
    
    #[test]
    fn test_expansion_gas_calculation() {
        let mut memory = Memory::new();
        
        // Expand to 32 bytes (1 word)
        let gas1 = memory.calculate_expansion_gas(32).unwrap();
        assert_eq!(gas1, 3); // First word costs 3 gas
        
        memory.resize(0, 32).unwrap();
        
        // Expand to 64 bytes (2 words)
        let gas2 = memory.calculate_expansion_gas(64).unwrap();
        // Cost for 2 words = 6, minus cost for 1 word = 3, so expansion = 3
        assert_eq!(gas2, 3);
    }
    
    #[test]
    fn test_out_of_gas() {
        let mut memory = Memory::new();
        let mut gas = 1; // Very low gas
        
        // Should fail with OutOfGas
        let result = memory.resize_with_gas(0, 1024, &mut gas);
        assert!(matches!(result, Err(EvmError::OutOfGas)));
    }
    
    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new();
        
        // Store and load
        memory.store(0, &[1, 2, 3, 4]).unwrap();
        let loaded = memory.load(0, 4).unwrap();
        assert_eq!(loaded, vec![1, 2, 3, 4]);
        
        // Load beyond stored data (should return zeros)
        let loaded_beyond = memory.load(100, 4).unwrap();
        assert_eq!(loaded_beyond, vec![0, 0, 0, 0]);
    }
}
