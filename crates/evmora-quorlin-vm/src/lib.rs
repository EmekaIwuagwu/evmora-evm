// Quorlin Native VM - Stack-based bytecode interpreter
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    U256(u128), // Simplified as u128
    Address([u8; 20]),
    Bool(bool),
    Bytes(Vec<u8>),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    // Stack operations
    Push = 0x00,
    Pop = 0x01,
    Dup = 0x02,
    Swap = 0x03,
    
    // Arithmetic
    Add = 0x10,
    Sub = 0x11,
    Mul = 0x12,
    Div = 0x13,
    
    // Comparison
    Eq = 0x20,
    Lt = 0x21,
    Gt = 0x22,
    
    // Control flow
    Jump = 0x30,
    JumpI = 0x31,
    Call = 0x32,
    Return = 0x33,
    
    // Storage
    SLoad = 0x40,
    SStore = 0x41,
    
    // System
    Caller = 0x50,
    Value = 0x51,
    Halt = 0xFF,
}

pub struct QuorlinVM {
    stack: Vec<Value>,
    memory: Vec<u8>,
    storage: HashMap<Vec<u8>, Vec<u8>>,
    pc: usize,
    code: Vec<u8>,
    caller: [u8; 20],
    value: u128,
}

impl QuorlinVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            memory: vec![0; 4096],
            storage: HashMap::new(),
            pc: 0,
            code: Vec::new(),
            caller: [0; 20],
            value: 0,
        }
    }

    pub fn execute(&mut self, bytecode: &[u8], caller: [u8; 20], value: u128) -> Result<Vec<Value>> {
        self.code = bytecode.to_vec();
        self.caller = caller;
        self.value = value;
        self.pc = 0;
        self.stack.clear();

        while self.pc < self.code.len() {
            let op = self.code[self.pc];
            self.pc += 1;

            match op {
                0x00 => self.op_push()?,      // PUSH
                0x01 => self.op_pop()?,        // POP
                0x10 => self.op_add()?,        // ADD
                0x11 => self.op_sub()?,        // SUB
                0x12 => self.op_mul()?,        // MUL
                0x13 => self.op_div()?,        // DIV
                0x20 => self.op_eq()?,         // EQ
                0x21 => self.op_lt()?,         // LT
                0x30 => self.op_jump()?,       // JUMP
                0x31 => self.op_jumpi()?,      // JUMPI
                0x33 => break,                 // RETURN
                0x40 => self.op_sload()?,      // SLOAD
                0x41 => self.op_sstore()?,     // SSTORE
                0x50 => self.op_caller()?,     // CALLER
                0xFF => break,                 // HALT
                _ => {}
            }
        }

        Ok(self.stack.clone())
    }

    fn op_push(&mut self) -> Result<()> {
        let value = self.read_u128()?;
        self.stack.push(Value::U256(value));
        Ok(())
    }

    fn op_pop(&mut self) -> Result<()> {
        self.stack.pop();
        Ok(())
    }

    fn op_add(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        self.stack.push(Value::U256(a.wrapping_add(b)));
        Ok(())
    }

    fn op_sub(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        self.stack.push(Value::U256(a.wrapping_sub(b)));
        Ok(())
    }

    fn op_mul(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        self.stack.push(Value::U256(a.wrapping_mul(b)));
        Ok(())
    }

    fn op_div(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        if b == 0 {
            self.stack.push(Value::U256(0));
        } else {
            self.stack.push(Value::U256(a / b));
        }
        Ok(())
    }

    fn op_eq(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        self.stack.push(Value::Bool(a == b));
        Ok(())
    }

    fn op_lt(&mut self) -> Result<()> {
        let b = self.pop_u128()?;
        let a = self.pop_u128()?;
        self.stack.push(Value::Bool(a < b));
        Ok(())
    }

    fn op_jump(&mut self) -> Result<()> {
        let dest = self.pop_u128()? as usize;
        self.pc = dest;
        Ok(())
    }

    fn op_jumpi(&mut self) -> Result<()> {
        let dest = self.pop_u128()? as usize;
        let cond = self.pop_u128()?;
        if cond != 0 {
            self.pc = dest;
        }
        Ok(())
    }

    fn op_sload(&mut self) -> Result<()> {
        let key = self.pop_bytes()?;
        let value = self.storage.get(&key).cloned().unwrap_or_default();
        let val_u128 = u128::from_le_bytes(value.as_slice().try_into().unwrap_or([0; 16]));
        self.stack.push(Value::U256(val_u128));
        Ok(())
    }

    fn op_sstore(&mut self) -> Result<()> {
        let key = self.pop_bytes()?;
        let value = self.pop_u128()?;
        self.storage.insert(key, value.to_le_bytes().to_vec());
        Ok(())
    }

    fn op_caller(&mut self) -> Result<()> {
        self.stack.push(Value::Address(self.caller));
        Ok(())
    }

    fn pop_u128(&mut self) -> Result<u128> {
        match self.stack.pop() {
            Some(Value::U256(v)) => Ok(v),
            Some(Value::Bool(b)) => Ok(if b { 1 } else { 0 }),
            _ => Ok(0),
        }
    }

    fn pop_bytes(&mut self) -> Result<Vec<u8>> {
        let val = self.pop_u128()?;
        Ok(val.to_le_bytes().to_vec())
    }

    fn read_u128(&mut self) -> Result<u128> {
        let mut bytes = [0u8; 16];
        for i in 0..16.min(self.code.len() - self.pc) {
            bytes[i] = self.code[self.pc + i];
        }
        self.pc += 16;
        Ok(u128::from_le_bytes(bytes))
    }

    pub fn get_storage(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }
}

impl Default for QuorlinVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut vm = QuorlinVM::new();
        // PUSH 5, PUSH 3, ADD, HALT
        let code = vec![
            0x00, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x00, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10, // ADD
            0xFF, // HALT
        ];
        
        let result = vm.execute(&code, [0; 20], 0).unwrap();
        assert_eq!(result.len(), 1);
        matches!(result[0], Value::U256(8));
    }
}
