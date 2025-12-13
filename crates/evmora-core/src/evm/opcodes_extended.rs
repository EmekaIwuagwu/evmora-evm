// Additional opcode implementations for CALL, CREATE, and signed arithmetic
// This file contains implementations to be integrated into executor.rs

use crate::evm::{Stack, Memory};
use crate::evm::context::ExecutionContext;
use evmora_utils::EvmError;
use evmora_plugins::StorageBackend;
use primitive_types::{U256, H160, H256};
use sha3::{Digest, Keccak256};

/// Helper struct for call context
#[derive(Debug, Clone)]
pub struct CallContext {
    pub caller: H160,
    pub address: H160,
    pub value: U256,
    pub call_data: Vec<u8>,
    pub depth: usize,
    pub is_static: bool,
    pub gas_limit: u64,
}

impl CallContext {
    pub fn new(caller: H160, address: H160, value: U256, call_data: Vec<u8>, depth: usize) -> Self {
        Self {
            caller,
            address,
            value,
            call_data,
            depth,
            is_static: false,
            gas_limit: 0,
        }
    }
}

/// CALL opcode (0xF1) implementation
pub fn op_call(
    stack: &mut Stack,
    memory: &mut Memory,
    gas_left: &mut u64,
    context: &ExecutionContext,
    depth: usize,
) -> Result<U256, EvmError> {
    // Stack: gas, address, value, argsOffset, argsSize, retOffset, retSize
    let gas_limit = stack.pop()?.as_u64();
    let address_u256 = stack.pop()?;
    let value = stack.pop()?;
    let args_offset = stack.pop()?.as_usize();
    let args_size = stack.pop()?.as_usize();
    let ret_offset = stack.pop()?.as_usize();
    let ret_size = stack.pop()?.as_usize();
    
    // Convert U256 address to H160
    let mut addr_bytes = [0u8; 32];
    address_u256.to_big_endian(&mut addr_bytes);
    let address = H160::from_slice(&addr_bytes[12..32]);
    
    // Check call depth limit (1024)
    if depth >= 1024 {
        return Ok(U256::zero()); // Failure
    }
    
    // Memory expansion for both input and output
    memory.resize_with_gas(args_offset, args_size, gas_left)?;
    memory.resize_with_gas(ret_offset, ret_size, gas_left)?;
    
    // Gas cost: 700 base + 9000 if value transfer + 25000 if creating new account
    let mut call_cost = 700u64;
    
    if !value.is_zero() {
        call_cost += 9000;
        // TODO: Check if account exists, add 25000 if creating new
    }
    
    // Deduct gas
    if *gas_left < call_cost {
        return Err(EvmError::OutOfGas);
    }
    *gas_left -= call_cost;
    
    // Calculate gas to forward (EIP-150: all but 1/64th)
    let gas_to_send = std::cmp::min(gas_limit, *gas_left - (*gas_left / 64));
    
    // TODO: Actually execute the call
    // For now, return success
    Ok(U256::one())
}

/// DELEGATECALL opcode (0xF4) implementation
pub fn op_delegatecall(
    stack: &mut Stack,
    memory: &mut Memory,
    gas_left: &mut u64,
    depth: usize,
) -> Result<U256, EvmError> {
    // Stack: gas, address, argsOffset, argsSize, retOffset, retSize
    let gas_limit = stack.pop()?.as_u64();
    let address_u256 = stack.pop()?;
    let args_offset = stack.pop()?.as_usize();
    let args_size = stack.pop()?.as_usize();
    let ret_offset = stack.pop()?.as_usize();
    let ret_size = stack.pop()?.as_usize();
    
    // Check depth
    if depth >= 1024 {
        return Ok(U256::zero());
    }
    
    // Memory expansion
    memory.resize_with_gas(args_offset, args_size, gas_left)?;
    memory.resize_with_gas(ret_offset, ret_size, gas_left)?;
    
    // Gas cost: 700 base
    const DELEGATECALL_GAS: u64 = 700;
    if *gas_left < DELEGATECALL_GAS {
        return Err(EvmError::OutOfGas);
    }
    *gas_left -= DELEGATECALL_GAS;
    
    // Calculate gas to forward
    let gas_to_send = std::cmp::min(gas_limit, *gas_left - (*gas_left / 64));
    
    // TODO: Execute with preserved context
    Ok(U256::one())
}

/// STATICCALL opcode (0xFA) implementation
pub fn op_staticcall(
    stack: &mut Stack,
    memory: &mut Memory,
    gas_left: &mut u64,
    depth: usize,
) -> Result<U256, EvmError> {
    // Stack: gas, address, argsOffset, argsSize, retOffset, retSize
    let gas_limit = stack.pop()?.as_u64();
    let address_u256 = stack.pop()?;
    let args_offset = stack.pop()?.as_usize();
    let args_size = stack.pop()?.as_usize();
    let ret_offset = stack.pop()?.as_usize();
    let ret_size = stack.pop()?.as_usize();
    
    // Check depth
    if depth >= 1024 {
        return Ok(U256::zero());
    }
    
    // Memory expansion
    memory.resize_with_gas(args_offset, args_size, gas_left)?;
    memory.resize_with_gas(ret_offset, ret_size, gas_left)?;
    
    // Gas cost: 700 base
    const STATICCALL_GAS: u64 = 700;
    if *gas_left < STATICCALL_GAS {
        return Err(EvmError::OutOfGas);
    }
    *gas_left -= STATICCALL_GAS;
    
    // Calculate gas to forward
    let gas_to_send = std::cmp::min(gas_limit, *gas_left - (*gas_left / 64));
    
    // TODO: Execute with static flag (no state modifications)
    Ok(U256::one())
}

/// CREATE opcode (0xF0) implementation
pub fn op_create(
    stack: &mut Stack,
    memory: &mut Memory,
    gas_left: &mut u64,
    context: &ExecutionContext,
    depth: usize,
) -> Result<U256, EvmError> {
    // Stack: value, offset, size
    let value = stack.pop()?;
    let offset = stack.pop()?.as_usize();
    let size = stack.pop()?.as_usize();
    
    // Check depth limit
    if depth >= 1024 {
        return Ok(U256::zero());
    }
    
    // Gas cost: 32000 base
    const CREATE_GAS: u64 = 32000;
    if *gas_left < CREATE_GAS {
        return Err(EvmError::OutOfGas);
    }
    *gas_left -= CREATE_GAS;
    
    // Get init code from memory
    memory.resize_with_gas(offset, size, gas_left)?;
    
    // TODO: Calculate new contract address
    // TODO: Execute init code
    // TODO: Store contract code
    
    // For now, return zero (failure)
    Ok(U256::zero())
}

/// CREATE2 opcode (0xF5) implementation
pub fn op_create2(
    stack: &mut Stack,
    memory: &mut Memory,
    gas_left: &mut u64,
    depth: usize,
) -> Result<U256, EvmError> {
    // Stack: value, offset, size, salt
    let value = stack.pop()?;
    let offset = stack.pop()?.as_usize();
    let size = stack.pop()?.as_usize();
    let salt = stack.pop()?;
    
    // Check depth
    if depth >= 1024 {
        return Ok(U256::zero());
    }
    
    // Gas cost: 32000 base
    const CREATE2_GAS: u64 = 32000;
    if *gas_left < CREATE2_GAS {
        return Err(EvmError::OutOfGas);
    }
    *gas_left -= CREATE2_GAS;
    
    // Get init code
    memory.resize_with_gas(offset, size, gas_left)?;
    
    // TODO: Calculate deterministic address using salt
    // address = keccak256(0xff ++ sender ++ salt ++ keccak256(init_code))[12:]
    
    Ok(U256::zero())
}

/// SDIV opcode (0x05) - Signed division
pub fn op_sdiv(stack: &mut Stack) -> Result<(), EvmError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    
    if b.is_zero() {
        stack.push(U256::zero())?;
        return Ok(());
    }
    
    // Convert to signed (two's complement)
    let a_negative = a.bit(255);
    let b_negative = b.bit(255);
    
    // Get absolute values
    let a_abs = if a_negative { (!a).overflowing_add(U256::one()).0 } else { a };
    let b_abs = if b_negative { (!b).overflowing_add(U256::one()).0 } else { b };
    
    // Divide absolute values
    let result_abs = a_abs / b_abs;
    
    // Apply sign
    let result = if a_negative != b_negative {
        // Result is negative
        (!result_abs).overflowing_add(U256::one()).0
    } else {
        result_abs
    };
    
    // Handle overflow case: MIN / -1 = MIN
    let min_value = U256::one() << 255;
    let neg_one = U256::MAX;
    if a == min_value && b == neg_one {
        stack.push(min_value)?;
    } else {
        stack.push(result)?;
    }
    
    Ok(())
}

/// SMOD opcode (0x07) - Signed modulo
pub fn op_smod(stack: &mut Stack) -> Result<(), EvmError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    
    if b.is_zero() {
        stack.push(U256::zero())?;
        return Ok(());
    }
    
    let a_negative = a.bit(255);
    let b_negative = b.bit(255);
    
    let a_abs = if a_negative { (!a).overflowing_add(U256::one()).0 } else { a };
    let b_abs = if b_negative { (!b).overflowing_add(U256::one()).0 } else { b };
    
    let result_abs = a_abs % b_abs;
    
    // Result takes sign of dividend (a)
    let result = if a_negative {
        (!result_abs).overflowing_add(U256::one()).0
    } else {
        result_abs
    };
    
    stack.push(result)?;
    Ok(())
}

/// SLT opcode (0x12) - Signed less than
pub fn op_slt(stack: &mut Stack) -> Result<(), EvmError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    
    let a_negative = a.bit(255);
    let b_negative = b.bit(255);
    
    let result = if a_negative && !b_negative {
        // a is negative, b is positive: a < b
        U256::one()
    } else if !a_negative && b_negative {
        // a is positive, b is negative: a >= b
        U256::zero()
    } else {
        // Same sign, compare normally
        if a < b { U256::one() } else { U256::zero() }
    };
    
    stack.push(result)?;
    Ok(())
}

/// SGT opcode (0x13) - Signed greater than
pub fn op_sgt(stack: &mut Stack) -> Result<(), EvmError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    
    let a_negative = a.bit(255);
    let b_negative = b.bit(255);
    
    let result = if a_negative && !b_negative {
        // a is negative, b is positive: a <= b
        U256::zero()
    } else if !a_negative && b_negative {
        // a is positive, b is negative: a > b
        U256::one()
    } else {
        // Same sign, compare normally
        if a > b { U256::one() } else { U256::zero() }
    };
    
    stack.push(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sdiv_positive() {
        let mut stack = Stack::new();
        // Stack is LIFO: push divisor first, then dividend
        stack.push(U256::from(3)).unwrap();  // divisor (second operand)
        stack.push(U256::from(10)).unwrap(); // dividend (first operand)
        op_sdiv(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(3));
    }
    
    #[test]
    fn test_sdiv_negative() {
        let mut stack = Stack::new();
        // -10 in two's complement
        let neg_10 = (!U256::from(10)).overflowing_add(U256::one()).0;
        stack.push(U256::from(3)).unwrap();  // divisor
        stack.push(neg_10).unwrap();         // dividend
        op_sdiv(&mut stack).unwrap();
        
        // Result should be -3
        let result = stack.pop().unwrap();
        let neg_3 = (!U256::from(3)).overflowing_add(U256::one()).0;
        assert_eq!(result, neg_3);
    }
    
    #[test]
    fn test_slt() {
        let mut stack = Stack::new();
        let neg_5 = (!U256::from(5)).overflowing_add(U256::one()).0;
        
        // -5 < 10 should be true
        // Stack is LIFO: push second operand first
        stack.push(U256::from(10)).unwrap(); // b (second operand)
        stack.push(neg_5).unwrap();          // a (first operand)
        op_slt(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::one());
    }
}

