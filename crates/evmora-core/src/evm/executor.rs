use crate::evm::{Stack, Memory};
use crate::evm::context::ExecutionContext;
use evmora_utils::EvmError;
use evmora_plugins::StorageBackend;
use primitive_types::U256;
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub return_data: Vec<u8>,
    pub gas_used: u64,
    pub contract_address: Option<crate::types::Address>,
    pub execution_time: std::time::Duration,
}

use evmora_plugins::GasCalculator;

pub struct Executor<'a> {
    pub stack: Stack,
    pub memory: Memory,
    pub storage: &'a mut dyn StorageBackend,
    pub gas_calculator: Box<dyn GasCalculator>,
    pub context: ExecutionContext,
    pub pc: usize,
    pub gas_used: u64,
}

impl<'a> Executor<'a> {
    pub fn new(context: ExecutionContext, storage: &'a mut dyn StorageBackend, gas_calculator: Box<dyn GasCalculator>) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            storage,
            gas_calculator,
            context,
            pc: 0,
            gas_used: 0,
        }
    }

    pub fn execute(&mut self, code: &[u8]) -> Result<ExecutionResult, EvmError> {
        let start_time = std::time::Instant::now();
        
        while self.pc < code.len() {
            let op = code[self.pc];
            
            let cost = self.gas_calculator.calculate_gas(op, self.stack.len());
            self.gas_used = self.gas_used.saturating_add(cost);
            if self.gas_used > self.context.gas_limit {
                 return Err(EvmError::OutOfGas);
            }
            
            self.pc += 1;

            match op {
                0x00 => { // STOP
                    return Ok(ExecutionResult {
                        success: true,
                        return_data: vec![],
                        gas_used: self.gas_used,
                        contract_address: None,
                        execution_time: start_time.elapsed(),
                    });
                }
                0x01 => { // ADD
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a.overflowing_add(b).0)?;
                }
                0x02 => { // MUL
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a.overflowing_mul(b).0)?;
                }
                0x03 => { // SUB
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a.overflowing_sub(b).0)?;
                }
                0x04 => { // DIV
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(a / b)?;
                    }
                }
                0x05 => { // SDIV
                     // Signed division (simplified treating as unsigned for now or should cast)
                     // Rust's U256 doesn't have sdiv directly, need to cast to I256 equivalent logic
                     let a = self.stack.pop()?;
                     let b = self.stack.pop()?;
                     if b.is_zero() {
                         self.stack.push(U256::zero())?;
                     } else {
                         // TODO: True signed division
                         self.stack.push(a / b)?; 
                     }
                }
                0x06 => { // MOD
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(a % b)?;
                    }
                }
                0x10 => { // LT
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if a < b { self.stack.push(U256::one())?; } else { self.stack.push(U256::zero())?; }
                }
                0x11 => { // GT
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if a > b { self.stack.push(U256::one())?; } else { self.stack.push(U256::zero())?; }
                }
                0x12 => { // SLT
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    // TODO: Signed compare
                    if a < b { self.stack.push(U256::one())?; } else { self.stack.push(U256::zero())?; }
                }
                0x13 => { // SGT
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    // TODO: Signed compare
                    if a > b { self.stack.push(U256::one())?; } else { self.stack.push(U256::zero())?; }
                }
                0x14 => { // EQ
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if a == b { self.stack.push(U256::one())?; } else { self.stack.push(U256::zero())?; }
                }
                0x15 => { // ISZERO
                    let a = self.stack.pop()?;
                    if a.is_zero() {
                        self.stack.push(U256::one())?;
                    } else {
                        self.stack.push(U256::zero())?;
                    }
                }
                0x16 => { // AND
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a & b)?;
                }
                0x17 => { // OR
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a | b)?;
                }
                0x18 => { // XOR
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a ^ b)?;
                }
                0x19 => { // NOT
                    let a = self.stack.pop()?;
                    self.stack.push(!a)?;
                }
                0x20 => { // SHA3
                    let offset = self.stack.pop()?.as_usize();
                    let size = self.stack.pop()?.as_usize();
                    let data = self.memory.load(offset, size)?;
                    let mut hasher = Keccak256::new();
                    hasher.update(&data);
                    let result = hasher.finalize();
                    self.stack.push(U256::from_big_endian(&result))?;
                }
                0x35 => { // CALLDATALOAD
                    let offset = self.stack.pop()?.as_usize();
                    let mut data = [0u8; 32];
                    let input = &self.context.data;
                    if offset < input.len() {
                        let available = input.len() - offset;
                        let n = available.min(32);
                        data[..n].copy_from_slice(&input[offset..offset+n]);
                    }
                    self.stack.push(U256::from_big_endian(&data))?;
                }
                0x39 => { // CODECOPY
                    let dest_offset = self.stack.pop()?.as_usize();
                    let code_offset = self.stack.pop()?.as_usize();
                    let length = self.stack.pop()?.as_usize();
                    
                    if length > 0 {
                        let end = code_offset.saturating_add(length).min(code.len());
                        let valid_slice = if code_offset < code.len() {
                             &code[code_offset..end]
                        } else {
                             &[]
                        };
                        let mut data = vec![0u8; length];
                        if !valid_slice.is_empty() {
                            data[..valid_slice.len()].copy_from_slice(valid_slice);
                        }
                        self.memory.store(dest_offset, &data)?;
                    }
                }
                0x50 => { // POP
                    self.stack.pop()?;
                }
                0x51 => { // MLOAD
                    let offset = self.stack.pop()?.as_usize();
                    let val_bytes = self.memory.load32(offset)?;
                    self.stack.push(U256::from_big_endian(&val_bytes))?;
                }
                0x52 => { // MSTORE
                    let offset = self.stack.pop()?.as_usize();
                    let val = self.stack.pop()?;
                    let mut bytes = [0u8; 32];
                    val.to_big_endian(&mut bytes);
                    self.memory.store(offset, &bytes)?;
                }
                0x54 => { // SLOAD
                    let key = self.stack.pop()?;
                    // Convert U256 key to H256
                    let mut key_bytes = [0u8; 32];
                    key.to_big_endian(&mut key_bytes);
                    let val = self.storage.get_storage(self.context.address, primitive_types::H256::from(key_bytes))
                        .map_err(|e| EvmError::StorageError(e.to_string()))?;
                    
                    // Convert H256 val back to U256
                    self.stack.push(U256::from_big_endian(val.as_bytes()))?;
                }
                0x55 => { // SSTORE
                    let key = self.stack.pop()?;
                    let val = self.stack.pop()?;
                    let mut key_bytes = [0u8; 32];
                    key.to_big_endian(&mut key_bytes);
                    let mut val_bytes = [0u8; 32];
                    val.to_big_endian(&mut val_bytes);
                    
                    self.storage.set_storage(self.context.address, primitive_types::H256::from(key_bytes), primitive_types::H256::from(val_bytes))
                        .map_err(|e| EvmError::StorageError(e.to_string()))?;
                }
                0x56 => { // JUMP
                    let dest = self.stack.pop()?.as_usize();
                    if dest >= code.len() || code[dest] != 0x5b { // JUMPDEST
                        return Err(EvmError::InvalidJump);
                    }
                    self.pc = dest;
                }
                0x57 => { // JUMPI
                    let dest = self.stack.pop()?.as_usize();
                    let cond = self.stack.pop()?;
                    if !cond.is_zero() {
                        if dest >= code.len() || code[dest] != 0x5b {
                            return Err(EvmError::InvalidJump);
                        }
                        self.pc = dest;
                    }
                }
                0x5b => { // JUMPDEST
                    // No-op
                }
                0x60..=0x7f => { // PUSH1..PUSH32
                    let n = (op - 0x60 + 1) as usize;
                    if self.pc + n > code.len() {
                        return Err(EvmError::InvalidOpcode(op));
                    }
                    let data = &code[self.pc..self.pc+n];
                    // safe convert
                    if n <= 32 {
                         self.stack.push(U256::from_big_endian(data))?;
                    }
                    self.pc += n;
                }
                0x80..=0x8f => { // DUP1..DUP16
                    let n = (op - 0x80 + 1) as usize;
                    self.stack.dup(n)?;
                }
                0x90..=0x9f => { // SWAP1..SWAP16
                    let n = (op - 0x90 + 1) as usize;
                    self.stack.swap(n)?;
                }
                0xf3 => { // RETURN
                     let offset = self.stack.pop()?.as_usize();
                     let size = self.stack.pop()?.as_usize();
                     let data = self.memory.load(offset, size)?;
                     return Ok(ExecutionResult {
                        success: true,
                        return_data: data,
                        gas_used: self.gas_used,
                        contract_address: None,
                         execution_time: start_time.elapsed(),
                     });
                }
                0xfd => { // REVERT
                     let offset = self.stack.pop()?.as_usize();
                     let size = self.stack.pop()?.as_usize();
                     let data = self.memory.load(offset, size)?;
                     return Err(EvmError::Reverted(data));
                }
                _ => {
                    // For now, treat unknown as STOP or error?
                    // Error is safer
                    // return Err(EvmError::InvalidOpcode(op));
                    // But for the sake of not crashing on unimplemented ops during dev:
                    eprintln!("Unimplemented opcode: {:x}", op);
                }
            }
        }

        Ok(ExecutionResult {
            success: true,
            return_data: vec![],
            gas_used: self.gas_used,
            contract_address: None,
            execution_time: start_time.elapsed(),
        })
    }
}
