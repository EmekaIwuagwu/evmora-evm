use crate::traits::{EvmPlugin, GasCalculator};
use anyhow::Result;

/// Standard gas calculator following Ethereum Yellow Paper specifications
#[derive(Debug)]
pub struct StandardGasCalculator;

impl StandardGasCalculator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StandardGasCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl EvmPlugin for StandardGasCalculator {
    fn name(&self) -> &str {
        "StandardGasCalculator"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl GasCalculator for StandardGasCalculator {
    fn calculate_gas(&self, opcode: u8, _stack_depth: usize) -> u64 {
        match opcode {
            // Zero gas
            0x00 => 0, // STOP
            
            // Base gas (3)
            0x01 => 3, // ADD
            0x02 => 3, // MUL
            0x03 => 3, // SUB
            0x04 => 5, // DIV
            0x05 => 5, // SDIV
            0x06 => 5, // MOD
            0x07 => 5, // SMOD
            0x08 => 8, // ADDMOD
            0x09 => 8, // MULMOD
            0x0a => 10, // EXP (base, additional per byte)
            0x0b => 5, // SIGNEXTEND
            
            // Comparison & bitwise (3)
            0x10 => 3, // LT
            0x11 => 3, // GT
            0x12 => 3, // SLT
            0x13 => 3, // SGT
            0x14 => 3, // EQ
            0x15 => 3, // ISZERO
            0x16 => 3, // AND
            0x17 => 3, // OR
            0x18 => 3, // XOR
            0x19 => 3, // NOT
            0x1a => 3, // BYTE
            0x1b => 3, // SHL
            0x1c => 3, // SHR
            0x1d => 3, // SAR
            
            // SHA3
            0x20 => 30, // SHA3 (base, additional per word)
            
            // Environmental (2-700)
            0x30 => 2, // ADDRESS
            0x31 => 700, // BALANCE (EIP-2929: 2600 cold, 100 warm)
            0x32 => 2, // ORIGIN
            0x33 => 2, // CALLER
            0x34 => 2, // CALLVALUE
            0x35 => 3, // CALLDATALOAD
            0x36 => 2, // CALLDATASIZE
            0x37 => 3, // CALLDATACOPY (base, additional per word)
            0x38 => 2, // CODESIZE
            0x39 => 3, // CODECOPY (base, additional per word)
            0x3a => 2, // GASPRICE
            0x3b => 700, // EXTCODESIZE (EIP-2929)
            0x3c => 700, // EXTCODECOPY (EIP-2929)
            0x3d => 2, // RETURNDATASIZE
            0x3e => 3, // RETURNDATACOPY
            0x3f => 700, // EXTCODEHASH (EIP-2929)
            
            // Block information (2-20)
            0x40 => 20, // BLOCKHASH
            0x41 => 2, // COINBASE
            0x42 => 2, // TIMESTAMP
            0x43 => 2, // NUMBER
            0x44 => 2, // DIFFICULTY/PREVRANDAO
            0x45 => 2, // GASLIMIT
            0x46 => 2, // CHAINID
            0x47 => 5, // SELFBALANCE
            0x48 => 2, // BASEFEE
            
            // Stack, memory, storage (2-5000)
            0x50 => 2, // POP
            0x51 => 3, // MLOAD
            0x52 => 3, // MSTORE
            0x53 => 3, // MSTORE8
            0x54 => 2100, // SLOAD (EIP-2929: 2100 cold, 100 warm)
            0x55 => 20000, // SSTORE (EIP-2929: complex, this is minimum)
            0x56 => 8, // JUMP
            0x57 => 10, // JUMPI
            0x58 => 2, // PC
            0x59 => 2, // MSIZE
            0x5a => 2, // GAS
            0x5b => 1, // JUMPDEST
            
            // PUSH1-PUSH32 (3)
            0x60..=0x7f => 3,
            
            // DUP1-DUP16 (3)
            0x80..=0x8f => 3,
            
            // SWAP1-SWAP16 (3)
            0x90..=0x9f => 3,
            
            // LOG0-LOG4 (375 base + 375 per topic + 8 per byte)
            0xa0 => 375, // LOG0
            0xa1 => 750, // LOG1
            0xa2 => 1125, // LOG2
            0xa3 => 1500, // LOG3
            0xa4 => 1875, // LOG4
            
            // System operations
            0xf0 => 32000, // CREATE
            0xf1 => 700, // CALL (base, complex actual cost)
            0xf2 => 700, // CALLCODE (deprecated)
            0xf3 => 0, // RETURN
            0xf4 => 700, // DELEGATECALL
            0xf5 => 32000, // CREATE2
            0xfa => 700, // STATICCALL
            0xfd => 0, // REVERT
            0xfe => 0, // INVALID
            0xff => 5000, // SELFDESTRUCT (EIP-6780: reduced)
            
            // Unknown opcode
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_gas() {
        let calc = StandardGasCalculator::new();
        
        assert_eq!(calc.calculate_gas(0x01, 0), 3); // ADD
        assert_eq!(calc.calculate_gas(0x02, 0), 3); // MUL
        assert_eq!(calc.calculate_gas(0x04, 0), 5); // DIV
    }

    #[test]
    fn test_storage_gas() {
        let calc = StandardGasCalculator::new();
        
        assert_eq!(calc.calculate_gas(0x54, 0), 2100); // SLOAD
        assert_eq!(calc.calculate_gas(0x55, 0), 20000); // SSTORE (minimum)
    }

    #[test]
    fn test_call_gas() {
        let calc = StandardGasCalculator::new();
        
        assert_eq!(calc.calculate_gas(0xf1, 0), 700); // CALL
        assert_eq!(calc.calculate_gas(0xf4, 0), 700); // DELEGATECALL
        assert_eq!(calc.calculate_gas(0xfa, 0), 700); // STATICCALL
    }

    #[test]
    fn test_create_gas() {
        let calc = StandardGasCalculator::new();
        
        assert_eq!(calc.calculate_gas(0xf0, 0), 32000); // CREATE
        assert_eq!(calc.calculate_gas(0xf5, 0), 32000); // CREATE2
    }
}
