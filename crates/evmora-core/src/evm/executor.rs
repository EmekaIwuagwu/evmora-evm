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
                0x05 => { // SDIV - Signed division
                    use crate::evm::opcodes_extended::op_sdiv;
                    op_sdiv(&mut self.stack)?;
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
                0x12 => { // SLT - Signed less than
                    use crate::evm::opcodes_extended::op_slt;
                    op_slt(&mut self.stack)?;
                }
                0x13 => { // SGT - Signed greater than
                    use crate::evm::opcodes_extended::op_sgt;
                    op_sgt(&mut self.stack)?;
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
                0x07 => { // SMOD - Signed modulo
                    use crate::evm::opcodes_extended::op_smod;
                    op_smod(&mut self.stack)?;
                }
                0x08 => { // ADDMOD
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let n = self.stack.pop()?;
                    if n.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let result = a.overflowing_add(b).0 % n;
                        self.stack.push(result)?;
                    }
                }
                0x09 => { // MULMOD
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let n = self.stack.pop()?;
                    if n.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let result = a.overflowing_mul(b).0 % n;
                        self.stack.push(result)?;
                    }
                }
                0x0a => { // EXP
                    let base = self.stack.pop()?;
                    let exponent = self.stack.pop()?;
                    self.stack.push(base.overflowing_pow(exponent).0)?;
                }
                0x0b => { // SIGNEXTEND
                    let b = self.stack.pop()?;
                    let x = self.stack.pop()?;
                    if b < U256::from(32) {
                        let bit_index = (8 * b.as_u32() + 7) as usize;
                        let bit = x.bit(bit_index);
                        let mask = (U256::one() << bit_index) - U256::one();
                        let result = if bit {
                            x | !mask
                        } else {
                            x & mask
                        };
                        self.stack.push(result)?;
                    } else {
                        self.stack.push(x)?;
                    }
                }
                0x1a => { // BYTE
                    let i = self.stack.pop()?;
                    let x = self.stack.pop()?;
                    if i < U256::from(32) {
                        let mut bytes = [0u8; 32];
                        x.to_big_endian(&mut bytes);
                        self.stack.push(U256::from(bytes[i.as_usize()]))?;
                    } else {
                        self.stack.push(U256::zero())?;
                    }
                }
                0x1b => { // SHL
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift >= U256::from(256) {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(value << shift.as_usize())?;
                    }
                }
                0x1c => { // SHR
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift >= U256::from(256) {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(value >> shift.as_usize())?;
                    }
                }
                0x1d => { // SAR (Arithmetic shift right)
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift >= U256::from(256) {
                        let sign_bit = value.bit(255);
                        if sign_bit {
                            self.stack.push(U256::max_value())?;
                        } else {
                            self.stack.push(U256::zero())?;
                        }
                    } else {
                        // TODO: Implement proper arithmetic shift
                        self.stack.push(value >> shift.as_usize())?;
                    }
                }
                0x20 => { // SHA3/KECCAK256
                    let offset = self.stack.pop()?.as_usize();
                    let size = self.stack.pop()?.as_usize();
                    let data = self.memory.load(offset, size)?;
                    let mut hasher = Keccak256::new();
                    hasher.update(&data);
                    let result = hasher.finalize();
                    self.stack.push(U256::from_big_endian(&result))?;
                }
                0x30 => { // ADDRESS
                    let addr_bytes = self.context.address.as_bytes();
                    self.stack.push(U256::from_big_endian(addr_bytes))?;
                }
                0x31 => { // BALANCE
                    let _addr = self.stack.pop()?;
                    // TODO: Query balance from state
                    self.stack.push(U256::zero())?;
                }
                0x32 => { // ORIGIN
                    let origin_bytes = self.context.origin.as_bytes();
                    self.stack.push(U256::from_big_endian(origin_bytes))?;
                }
                0x33 => { // CALLER
                    let caller_bytes = self.context.caller.as_bytes();
                    self.stack.push(U256::from_big_endian(caller_bytes))?;
                }
                0x34 => { // CALLVALUE
                    self.stack.push(self.context.value)?;
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
                0x36 => { // CALLDATASIZE
                    self.stack.push(U256::from(self.context.data.len()))?;
                }
                0x37 => { // CALLDATACOPY
                    let dest_offset = self.stack.pop()?.as_usize();
                    let data_offset = self.stack.pop()?.as_usize();
                    let length = self.stack.pop()?.as_usize();
                    
                    if length > 0 {
                        let input = &self.context.data;
                        let end = data_offset.saturating_add(length).min(input.len());
                        let valid_slice = if data_offset < input.len() {
                            &input[data_offset..end]
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
                0x38 => { // CODESIZE
                    self.stack.push(U256::from(code.len()))?;
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
                0x3a => { // GASPRICE
                    self.stack.push(U256::from(self.context.gas_price))?;
                }
                0x3b => { // EXTCODESIZE
                    let _addr = self.stack.pop()?;
                    // TODO: Query code size from state
                    self.stack.push(U256::zero())?;
                }
                0x3d => { // RETURNDATASIZE
                    // TODO: Track return data
                    self.stack.push(U256::zero())?;
                }
                0x3f => { // EXTCODEHASH
                    let _addr = self.stack.pop()?;
                    // TODO: Query code hash from state
                    self.stack.push(U256::zero())?;
                }
                0x40 => { // BLOCKHASH
                    let _block_number = self.stack.pop()?;
                    // TODO: Query block hash
                    self.stack.push(U256::zero())?;
                }
                0x41 => { // COINBASE
                    let coinbase_bytes = self.context.coinbase.as_bytes();
                    self.stack.push(U256::from_big_endian(coinbase_bytes))?;
                }
                0x42 => { // TIMESTAMP
                    self.stack.push(U256::from(self.context.timestamp))?;
                }
                0x43 => { // NUMBER
                    self.stack.push(U256::from(self.context.block_number))?;
                }
                0x44 => { // DIFFICULTY/PREVRANDAO
                    self.stack.push(self.context.difficulty)?;
                }
                0x45 => { // GASLIMIT
                    self.stack.push(U256::from(self.context.gas_limit))?;
                }
                0x46 => { // CHAINID
                    self.stack.push(U256::from(self.context.chain_id))?;
                }
                0x47 => { // SELFBALANCE
                    // TODO: Query self balance
                    self.stack.push(U256::zero())?;
                }
                0x48 => { // BASEFEE
                    self.stack.push(U256::from(self.context.base_fee))?;
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
                0x53 => { // MSTORE8
                    let offset = self.stack.pop()?.as_usize();
                    let val = self.stack.pop()?;
                    let byte = (val.low_u32() & 0xFF) as u8;
                    self.memory.store(offset, &[byte])?;
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
                0x58 => { // PC
                    self.stack.push(U256::from(self.pc - 1))?;
                }
                0x59 => { // MSIZE
                    self.stack.push(U256::from(self.memory.size()))?;
                }
                0x5a => { // GAS
                    let remaining_gas = self.context.gas_limit.saturating_sub(self.gas_used);
                    self.stack.push(U256::from(remaining_gas))?;
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
                0xa0..=0xa4 => { // LOG0..LOG4
                    let offset = self.stack.pop()?.as_usize();
                    let size = self.stack.pop()?.as_usize();
                    let topic_count = (op - 0xa0) as usize;
                    let mut _topics = Vec::new();
                    for _ in 0..topic_count {
                        _topics.push(self.stack.pop()?);
                    }
                    let _data = self.memory.load(offset, size)?;
                    // TODO: Emit log event
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
                0xf0 => { // CREATE
                    use crate::evm::opcodes_extended::op_create;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_create(&mut self.stack, &mut self.memory, &mut gas_left, &self.context, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xf1 => { // CALL
                    use crate::evm::opcodes_extended::op_call;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_call(&mut self.stack, &mut self.memory, &mut gas_left, &self.context, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xf2 => { // CALLCODE (deprecated but included for compatibility)
                    use crate::evm::opcodes_extended::op_call;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_call(&mut self.stack, &mut self.memory, &mut gas_left, &self.context, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xf4 => { // DELEGATECALL
                    use crate::evm::opcodes_extended::op_delegatecall;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_delegatecall(&mut self.stack, &mut self.memory, &mut gas_left, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xf5 => { // CREATE2
                    use crate::evm::opcodes_extended::op_create2;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_create2(&mut self.stack, &mut self.memory, &mut gas_left, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xfa => { // STATICCALL
                    use crate::evm::opcodes_extended::op_staticcall;
                    let mut gas_left = self.context.gas_limit - self.gas_used;
                    let result = op_staticcall(&mut self.stack, &mut self.memory, &mut gas_left, 0)?;
                    self.gas_used = self.context.gas_limit - gas_left;
                    self.stack.push(result)?;
                }
                0xff => { // SELFDESTRUCT
                    let _beneficiary = self.stack.pop()?;
                    // TODO: Implement selfdestruct logic (mark for deletion, transfer balance)
                    return Ok(ExecutionResult {
                        success: true,
                        return_data: vec![],
                        gas_used: self.gas_used,
                        contract_address: None,
                        execution_time: start_time.elapsed(),
                    });
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
