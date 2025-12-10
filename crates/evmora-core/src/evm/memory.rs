use evmora_utils::EvmError;


#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn resize(&mut self, offset: usize, size: usize) -> Result<(), EvmError> {
        if size == 0 {
            return Ok(());
        }
        let end = offset.checked_add(size).ok_or(EvmError::MemoryViolation)?;
        
        if end > self.data.len() {
            // Round up to multiple of 32
            let new_size = (end + 31) / 32 * 32;
            self.data.resize(new_size, 0);
        }
        Ok(())
    }

    pub fn load(&self, offset: usize, size: usize) -> Result<Vec<u8>, EvmError> {
        if offset + size > self.data.len() {
             // In EVM, reading past end returns 0s, usually we just ensure capacity first or handle it here
             // For strict implementation, we should expect resize to have been called if intent was to persist, 
             // but 'mload' might read unallocated space as 0.
             // We'll simplisticly return 0-padded.
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
        // Optimistic load
        if offset + 32 > self.data.len() {
            // Handle padding
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
    
    pub fn store8(&mut self, offset: usize, value: u8) -> Result<(), EvmError> {
        self.resize(offset, 1)?;
        self.data[offset] = value;
        Ok(())
    }
}
