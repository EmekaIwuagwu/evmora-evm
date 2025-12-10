use primitive_types::U256;
use evmora_utils::EvmError;

#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<U256>,
    limit: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024),
            limit: 1024,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, value: U256) -> Result<(), EvmError> {
        if self.data.len() >= self.limit {
            return Err(EvmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256, EvmError> {
        self.data.pop().ok_or(EvmError::StackUnderflow)
    }

    pub fn peek(&self, idx: usize) -> Result<U256, EvmError> {
        if idx >= self.data.len() {
            return Err(EvmError::StackUnderflow);
        }
        Ok(self.data[self.data.len() - 1 - idx])
    }

    pub fn swap(&mut self, idx: usize) -> Result<(), EvmError> {
        if idx >= self.data.len() {
            return Err(EvmError::StackUnderflow);
        }
        let top_idx = self.data.len() - 1;
        let swap_idx = top_idx - idx;
        self.data.swap(top_idx, swap_idx);
        Ok(())
    }

    pub fn dup(&mut self, idx: usize) -> Result<(), EvmError> {
        if self.data.len() >= self.limit {
            return Err(EvmError::StackOverflow);
        }
        let val = self.peek(idx - 1)?;
        self.push(val)
    }
}
