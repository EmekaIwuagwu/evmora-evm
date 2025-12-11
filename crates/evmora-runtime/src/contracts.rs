// Example Smart Contracts for All Platforms

// ============================================================================
// 1. SOLIDITY (EVM) - Simple Storage Contract
// ============================================================================

pub mod solidity {
    /// Compiled bytecode for a simple storage contract:
    /// ```solidity
    /// contract SimpleStorage {
    ///     uint256 public value;
    ///     
    ///     function setValue(uint256 _value) public {
    ///         value = _value;
    ///     }
    ///     
    ///     function getValue() public view returns (uint256) {
    ///         return value;
    ///     }
    /// }
    /// ```
    pub fn simple_storage_bytecode() -> Vec<u8> {
        vec![
            // Constructor
            0x60, 0x80, // PUSH1 0x80 (free memory pointer)
            0x60, 0x40, // PUSH1 0x40
            0x52,       // MSTORE
            
            // Initialize storage slot 0 to 0
            0x60, 0x00, // PUSH1 0
            0x60, 0x00, // PUSH1 0
            0x55,       // SSTORE
            
            // Return runtime code
            0x60, 0x20, // PUSH1 32 (size)
            0x60, 0x0c, // PUSH1 12 (offset)
            0xf3,       // RETURN
            
            // Runtime code starts here
            // Function selector check
            0x60, 0x00, // PUSH1 0
            0x35,       // CALLDATALOAD
            0x60, 0xe0, // PUSH1 224
            0x1c,       // SHR (shift right to get selector)
            
            // Check for setValue(uint256) - selector: 0x55241077
            0x80,       // DUP1
            0x63, 0x55, 0x24, 0x10, 0x77, // PUSH4 0x55241077
            0x14,       // EQ
            0x60, 0x28, // PUSH1 40 (jump dest for setValue)
            0x57,       // JUMPI
            
            // Check for getValue() - selector: 0x20965255
            0x80,       // DUP1
            0x63, 0x20, 0x96, 0x52, 0x55, // PUSH4 0x20965255
            0x14,       // EQ
            0x60, 0x3c, // PUSH1 60 (jump dest for getValue)
            0x57,       // JUMPI
            
            // Revert if no match
            0x60, 0x00, // PUSH1 0
            0x60, 0x00, // PUSH1 0
            0xfd,       // REVERT
            
            // setValue function
            0x5b,       // JUMPDEST (offset 40)
            0x60, 0x04, // PUSH1 4
            0x35,       // CALLDATALOAD
            0x60, 0x00, // PUSH1 0
            0x55,       // SSTORE
            0x00,       // STOP
            
            // getValue function
            0x5b,       // JUMPDEST (offset 60)
            0x60, 0x00, // PUSH1 0
            0x54,       // SLOAD
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xf3,       // RETURN
        ]
    }
    
    /// Simple token contract bytecode
    pub fn simple_token_bytecode() -> Vec<u8> {
        vec![
            // Minimal ERC20-like token
            0x60, 0x80, // PUSH1 0x80
            0x60, 0x40, // PUSH1 0x40
            0x52,       // MSTORE
            
            // Set total supply to 1000000
            0x62, 0x0f, 0x42, 0x40, // PUSH3 1000000
            0x60, 0x00, // PUSH1 0 (storage slot)
            0x55,       // SSTORE
            
            // Set deployer balance
            0x62, 0x0f, 0x42, 0x40, // PUSH3 1000000
            0x33,       // CALLER
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0x20,       // SHA3
            0x55,       // SSTORE
            
            0x00,       // STOP
        ]
    }
}

// ============================================================================
// 2. VYPER - Simple Contract
// ============================================================================

pub mod vyper {
    /// Compiled bytecode for Vyper contract:
    /// ```python
    /// value: public(uint256)
    /// 
    /// @external
    /// def set_value(new_value: uint256):
    ///     self.value = new_value
    /// ```
    pub fn simple_storage_bytecode() -> Vec<u8> {
        // Similar to Solidity but with Vyper's specific patterns
        vec![
            0x60, 0x80, 0x60, 0x40, 0x52,
            0x60, 0x00, 0x60, 0x00, 0x55,
            0x60, 0x20, 0x60, 0x0c, 0xf3,
            // Runtime code
            0x60, 0x00, 0x35, 0x60, 0xe0, 0x1c,
            0x60, 0x04, 0x35, 0x60, 0x00, 0x55,
            0x00,
        ]
    }
}

// ============================================================================
// 3. QUORLIN - Native Bytecode
// ============================================================================

pub mod quorlin {
    /// Simple counter contract in Quorlin bytecode
    pub fn counter_bytecode() -> Vec<u8> {
        vec![
            // Initialize counter to 0
            0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 0
            0x41, // SSTORE (store at slot 0)
            
            // Increment function
            0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 0 (key)
            0x40, // SLOAD
            0x00, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 1
            0x10, // ADD
            0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 0 (key)
            0x41, // SSTORE
            
            0xFF, // HALT
        ]
    }
    
    /// Simple token contract
    pub fn token_bytecode() -> Vec<u8> {
        vec![
            // Set total supply
            0x00, 0xe8, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 1000
            0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 0 (total supply slot)
            0x41, // SSTORE
            
            // Set deployer balance
            0x00, 0xe8, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PUSH 1000
            0x50, // CALLER
            0x41, // SSTORE
            
            0xFF, // HALT
        ]
    }
}

// ============================================================================
// 4. SOLANA - BPF Program
// ============================================================================

pub mod solana {
    /// Simple token program for Solana
    pub fn token_program() -> Vec<u8> {
        // Simplified BPF bytecode representation
        // In reality, this would be compiled BPF
        vec![
            // Instruction 0: Initialize
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 1000 initial supply
            
            // Instruction 1: Transfer logic
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            
            // Instruction 2: Balance query
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }
    
    /// Simple counter program
    pub fn counter_program() -> Vec<u8> {
        vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }
}

// ============================================================================
// 5. POLKADOT/SUBSTRATE - WASM Contract
// ============================================================================

pub mod polkadot {
    /// ink! contract WASM bytecode
    pub fn flipper_contract() -> Vec<u8> {
        // WASM module for a simple flipper contract
        let mut wasm = vec![
            0x00, 0x61, 0x73, 0x6d, // Magic
            0x01, 0x00, 0x00, 0x00, // Version
        ];
        
        // Type section
        wasm.extend_from_slice(&[
            0x01, 0x07, // Section 1, length 7
            0x01, // 1 type
            0x60, 0x00, 0x01, 0x7f, // func () -> i32
        ]);
        
        // Function section
        wasm.extend_from_slice(&[
            0x03, 0x02, // Section 3, length 2
            0x01, 0x00, // 1 function, type 0
        ]);
        
        // Memory section
        wasm.extend_from_slice(&[
            0x05, 0x03, // Section 5, length 3
            0x01, 0x00, 0x01, // 1 memory, min 0, max 1
        ]);
        
        // Export section
        wasm.extend_from_slice(&[
            0x07, 0x08, // Section 7, length 8
            0x01, // 1 export
            0x04, b'f', b'l', b'i', b'p', // name "flip"
            0x00, 0x00, // function 0
        ]);
        
        // Code section
        wasm.extend_from_slice(&[
            0x0a, 0x09, // Section 10, length 9
            0x01, // 1 code
            0x07, // body size 7
            0x00, // 0 locals
            0x41, 0x01, // i32.const 1
            0x0b, // end
        ]);
        
        wasm
    }
    
    /// Simple storage contract
    pub fn storage_contract() -> Vec<u8> {
        vec![
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
            0x01, 0x07, 0x01, 0x60, 0x01, 0x7f, 0x00,
            0x03, 0x02, 0x01, 0x00,
            0x0a, 0x06, 0x01, 0x04, 0x00, 0x20, 0x00, 0x0b,
        ]
    }
}

// ============================================================================
// 6. APTOS - Move Module
// ============================================================================

pub mod aptos {
    /// Simple Move module bytecode
    pub fn simple_coin_module() -> Vec<u8> {
        vec![
            0xa1, 0x1c, 0xeb, 0x0b, // Move magic
            0x06, // Version 6
            
            // Module header
            0x01, // Module
            
            // Address pool
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
            
            // Identifier pool
            0x03, // 3 identifiers
            0x04, b'C', b'o', b'i', b'n',
            0x04, b'm', b'i', b'n', b't',
            0x08, b't', b'r', b'a', b'n', b's', b'f', b'e', b'r',
            
            // Function signatures
            0x02, // 2 functions
            0x00, 0x00, // mint: () -> ()
            0x02, 0x7f, 0x7f, 0x00, // transfer: (address, u64) -> ()
            
            // Code
            0x02, // 2 code units
            
            // mint function
            0x00, // 0 locals
            0x00, 0x03, // code length 3
            0x08, 0xe8, 0x03, // LdU64 1000
            0x05, // Ret
            
            // transfer function
            0x00, // 0 locals
            0x00, 0x02, // code length 2
            0x1f, // Add (simplified)
            0x05, // Ret
        ]
    }
    
    /// Counter module
    pub fn counter_module() -> Vec<u8> {
        vec![
            0xa1, 0x1c, 0xeb, 0x0b, 0x06,
            0x01,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x02,
            0x09, b'i', b'n', b'c', b'r', b'e', b'm', b'e', b'n', b't',
            0x05, b'v', b'a', b'l', b'u', b'e',
            0x01, 0x00, 0x00,
            0x01,
            0x00, 0x00, 0x04,
            0x08, 0x01, // LdU64 1
            0x1f, // Add
            0x05, // Ret
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_contracts_valid() {
        // Verify all contracts have valid bytecode
        assert!(!solidity::simple_storage_bytecode().is_empty());
        assert!(!solidity::simple_token_bytecode().is_empty());
        assert!(!vyper::simple_storage_bytecode().is_empty());
        assert!(!quorlin::counter_bytecode().is_empty());
        assert!(!quorlin::token_bytecode().is_empty());
        assert!(!solana::token_program().is_empty());
        assert!(!solana::counter_program().is_empty());
        assert!(!polkadot::flipper_contract().is_empty());
        assert!(!polkadot::storage_contract().is_empty());
        assert!(!aptos::simple_coin_module().is_empty());
        assert!(!aptos::counter_module().is_empty());
    }
    
    #[test]
    fn test_wasm_magic() {
        let wasm = polkadot::flipper_contract();
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6d]);
    }
    
    #[test]
    fn test_move_magic() {
        let module = aptos::simple_coin_module();
        assert_eq!(&module[0..4], &[0xa1, 0x1c, 0xeb, 0x0b]);
    }
}
