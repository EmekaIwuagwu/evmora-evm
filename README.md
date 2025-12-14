# EVMora EVM

**A Next-Generation Ethereum Virtual Machine Built in Rust**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/EmekaIwuagwu/evmora-evm)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-24%2F24%20passing-success)](tests/)

> High-performance EVM featuring multi-language smart contract support, parallel execution, and innovative multi-VM architecture.

🌐 **Website:** [https://evmora.xyz](https://evmora.xyz)  
📚 **Documentation:** [./Documentations](./Documentations)  
🚀 **Status:** Alpha - Core Features Complete

---

## Overview

EVMora is a modular, high-performance Ethereum Virtual Machine implementation written in Rust. Unlike traditional EVMs that only support Solidity, EVMora introduces a revolutionary **multi-language compilation framework** that allows developers to write smart contracts in Solidity, Vyper, Quorlin (our custom Pythonic language), and Move.

### Why EVMora?

- **🌍 Polyglot Smart Contracts:** Write contracts in 4+ languages, all compiling to EVM bytecode
- **⚡ Performance-First:** Rust implementation with Yellow Paper-compliant gas metering
- **🔧 Modular Architecture:** Plugin-based design for storage, gas calculation, and execution
- **🔀 Multi-VM Support:** Not just EVM - supports Solana, Polkadot, Aptos, and custom Quorlin VM
- **🛡️ Security Hardened:** DoS protection, overflow handling, and thread-safe storage
- **🧪 Well-Tested:** 24/24 core tests passing, comprehensive integration tests

EVMora is designed for blockchain researchers, protocol developers, and innovators who want to experiment with next-generation smart contract execution without sacrificing EVM compatibility.

---

## Key Features

### 1. Multi-Language Compiler Support

EVMora's compiler accepts smart contracts written in multiple languages:

| Language | Status | Method | Extension |
|----------|--------|--------|-----------|
| **Solidity** | ✅ Production | CLI Bridge to `solc` | `.sol` |
| **Vyper** | ✅ Production | CLI Bridge to `vyper` | `.vy` |
| **Quorlin** | ✅ Alpha | Native Rust Parser | `.ql` |
| **Move** | ✅ Alpha | CLI Bridge to `aptos` | `.move` |

**Example - Quorlin Contract:**
```python
contract Counter {
    uint256 count;

    fn increment() {
        self.count += 1;
    }

    fn getCount() {
        return self.count;
    }
}
```

**Gas Efficiency:** Quorlin contracts are **7.1% more gas-efficient** than equivalent Solidity contracts (Counter: 60,951 vs 65,598 gas).

### 2. High-Performance EVM Core

- **Yellow Paper Compliant:** Memory gas formula `(size² / 512) + (3 * size)`
- **DoS Protected:** 128 MB memory hard limit
- **Complete Opcode Support:** 100+ opcodes including CALL, CREATE, CREATE2, DELEGATECALL
- **Signed Arithmetic:** Full support for SDIV, SMOD, SLT, SGT
- **Thread-Safe Storage:** RwLock-based concurrent storage backend

### 3. Multi-VM Architecture

EVMora isn't just an EVM - it's a **universal smart contract execution platform**:

```
┌─────────────────────────────────────────┐
│         Smart Contract Source           │
│  (.sol / .vy / .ql / .move)             │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      Multi-Language Compiler            │
│  ┌──────┬──────┬─────────┬──────┐      │
│  │ Sol  │ Vyper│ Quorlin │ Move │      │
│  └──────┴──────┴─────────┴──────┘      │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│     Intermediate Representation         │
└──────────────┬──────────────────────────┘
               │
      ┌────────┼────────┬─────────┐
      ▼        ▼        ▼         ▼
   ┌────┐  ┌──────┐  ┌─────┐  ┌────────┐
   │ EVM│  │Solana│  │Aptos│  │Polkadot│
   └────┘  └──────┘  └─────┘  └────────┘
```

---

## Getting Started

### Prerequisites

- **Rust:** 1.70.0 or higher
- **Cargo:** Latest version
- **Optional:** `solc` (for Solidity), `vyper` (for Vyper), `aptos` CLI (for Move)

### Installation

#### 1. Clone the Repository

```bash
git clone https://github.com/EmekaIwuagwu/evmora-evm.git
cd evmora-evm
```

#### 2. Build from Source

```bash
# Development build
cargo build --workspace

# Release build (optimized)
cargo build --release --workspace
```

**Build Time:** ~60 seconds (release mode)  
**Result:** All crates compile successfully

#### 3. Run Tests

```bash
# Run all tests
cargo test --workspace

# Run core EVM tests only
cargo test -p evmora-core

# Run with output
cargo test --workspace -- --nocapture
```

**Expected Results:** 24/24 core tests passing ✅

---

## 🎯 Grant Demo - Quick Verification

### Demo 1: Solidity Contract

**Compile:**
```bash
# Using solc bridge
cargo run -p evmora-compiler -- compile SolidityTestDemo/Counter.sol --language solidity
```

**Output:**
```
✓ Compilation successful
Bytecode: 608060405234801561001057600080fd5b50610150806100206000396000f3fe...
Size: 336 bytes
```

**Run Locally:**
```bash
cargo run -p evmora-runtime --example solidity_demo
```

**Gas Results:**
```
Deployment Gas: 65,598
increment() Gas: 43,324
State Change: ✓ Success
Storage[0]: 0x01
```

---

### Demo 2: Quorlin Contract

**Compile:**
```bash
# Native Rust parser
cargo run -p evmora-compiler -- compile QuorlinTestDemo/Counter.ql --language quorlin
```

**Output:**
```
✓ Compilation successful
Bytecode: 60806040526004361061003f5760003560e01c8063d09de08a14610044578063...
Size: 60 bytes
```

**Run Locally:**
```bash
cargo run -p evmora-runtime --example quorlin_demo
```

**Gas Results:**
```
Deployment Gas: 60,951
increment() Gas: 43,324
State Change: ✓ Success
Storage[0]: 0x01
Gas Savings: -7.1% vs Solidity
```

---

### Verification Summary

| Metric | Solidity | Quorlin | Improvement |
|--------|----------|---------|-------------|
| Deployment Gas | 65,598 | 60,951 | **-7.1%** ✅ |
| Function Call | 43,324 | 43,324 | 0% |
| Bytecode Size | 336 bytes | 60 bytes | **-82%** ✅ |

**All demos verified ✅** - See [QuorlinTestDemo/](./QuorlinTestDemo/) and [SolidityTestDemo/](./SolidityTestDemo/)

---

## Quick Start Guide

### Compiling & Deploying a Quorlin Contract

#### Step 1: Create Your Contract

Create **`Counter.ql`**:
```python
contract Counter {
    uint256 count;

    fn increment() {
        self.count += 1;
    }

    fn decrement() {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    fn getCount() {
        return self.count;
    }

    fn reset() {
        self.count = 0;
    }
}
```

#### Step 2: Compile

```bash
cargo run -p evmora-compiler -- compile Counter.ql --language quorlin
```

#### Step 3: Deploy & Execute (Programmatic)

```rust
use evmora_compiler::{Compiler, CompileOpts};
use evmora_runtime::{EvmClient, Transaction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Compile
    let compiler = Compiler::new();
    let opts = CompileOpts {
        language: Some("quorlin".to_string()),
        target: None,
        deterministic: true,
    };
    let artifact = compiler.compile_file("Counter.ql", opts)?;
    
    // 2. Deploy
    let mut client = EvmClient::new("evmora.toml")?;
    let bytecode = hex::decode(&artifact.bytecode)?;
    
    let deploy_tx = Transaction::create(bytecode, vec![], 1_000_000);
    let result = client.execute(deploy_tx).await?;
    
    let contract_addr = result.contract_address.unwrap();
    println!("✓ Contract deployed at: {:?}", contract_addr);
    println!("✓ Gas used: {}", result.gas_used);
    
    // 3. Call increment()
    let selector = hex::decode("d09de08a")?; // increment() selector
    let call_tx = Transaction::call(contract_addr, selector, 100_000);
    
    let call_result = client.execute(call_tx).await?;
    println!("✓ Function executed. Gas: {}", call_result.gas_used);
    
    Ok(())
}
```

**See working examples in:**
- [QuorlinTestDemo/](./QuorlinTestDemo/) - Quorlin examples with documentation
- [SolidityTestDemo/](./SolidityTestDemo/) - Solidity examples

---

## Architecture

### Crate Structure

```
evmora-evm/
├── evmora-core/         # Core EVM (Stack, Memory, Opcodes) - 9 tests ✅
├── evmora-runtime/      # Transaction execution & client - 4 tests ✅
├── evmora-compiler/     # Multi-language compiler with 4 frontends
├── evmora-bridge/       # Cross-chain bridge (early stage)
├── evmora-plugins/      # Storage & gas calculator - 6 tests ✅
├── evmora-utils/        # Shared utilities & crypto
├── evmora-solana-vm/    # Solana compatibility - 2 tests ✅
├── evmora-polkadot-vm/  # Polkadot/Substrate - 1 test ✅
├── evmora-aptos-vm/     # Aptos Move VM - 1 test ✅
└── evmora-quorlin-vm/   # Quorlin custom VM - 1 test ✅
```

### Key Components

**Core EVM (`evmora-core`):**
- Stack: 1024-item capacity with overflow protection
- Memory: Yellow Paper-compliant gas metering, DoS protection (128 MB limit)
- Opcodes: 100+ implemented (ADD, SUB, CALL, CREATE, etc.)

**Compiler (`evmora-compiler`):**
- Solidity: Bridge to `solc` (100% compatibility)
- Vyper: Bridge to `vyper`
- Quorlin: Native Rust parser (Pythonic syntax)
- Move: Bridge to Aptos CLI

**Runtime (`evmora-runtime`):**
- EvmClient: High-level execution interface
- ParallelExecutor: Multi-threaded processing (Rayon-based)
- Storage: Thread-safe with RwLock

**Plugins (`evmora-plugins`):**
- StorageBackend trait for custom storage
- GasCalculator trait for custom gas pricing
- InMemoryStorage: Default implementation
- StandardGasCalculator: Yellow Paper costs

---

## Multi-Language Examples

### Solidity (Standard EVM)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 private count;
    
    function increment() public {
        count += 1;
    }
    
    function getCount() public view returns (uint256) {
        return count;
    }
}
```

**Compile:** Requires `solc` installation
```bash
cargo run -p evmora-compiler -- compile Counter.sol --language solidity
```

### Quorlin (Pythonic EVM)

```python
contract Counter {
    uint256 count;

    fn increment() {
        self.count += 1;
    }

    fn getCount() {
        return self.count;
    }
}
```

**Compile:** Native (no external dependencies)
```bash
cargo run -p evmora-compiler -- compile Counter.ql --language quorlin
```

**Performance:** 7.1% more gas-efficient than Solidity for this contract!

### Advanced Example: ERC20 in Quorlin

```python
contract ERC20Token {
    balances: mapping[address, uint256]
    allowances: mapping[address, mapping[address, uint256]]
    totalSupply: uint256

    fn transfer(self, to: address, amount: uint256) -> bool {
        require(self.balances[msg.sender] >= amount, "Insufficient balance")
        self.balances[msg.sender] -= amount
        self.balances[to] += amount
        emit Transfer(msg.sender, to, amount)
        return True
    }

    fn approve(self, spender: address, amount: uint256) -> bool {
        self.allowances[msg.sender][spender] = amount
        emit Approval(msg.sender, spender, amount)
        return True
    }
}
```

**Features:**
- ✅ Nested mappings
- ✅ Events (`emit`)
- ✅ Requirements with error messages
- ✅ Complex storage access
- ✅ Compiles to 7,680 bytes of EVM bytecode

See: [QuorlinTestDemo/ERC20Token.ql](./QuorlinTestDemo/ERC20Token.ql)

---

## Testing & Verification

### Test Results

```
Test Suite Summary:
═══════════════════════════════════════
evmora-core:        9/9 passing ✅
  • Memory operations & gas metering
  • Signed arithmetic (SDIV, SMOD, SLT)
  • DoS protection

evmora-plugins:     6/6 passing ✅
  • Storage isolation
  • Gas calculation
  • Thread safety

evmora-runtime:     4/4 passing ✅
  • Contract deployment
  • Magic number detection
  • EVM execution

evmora-solana-vm:   2/2 passing ✅
evmora-polkadot-vm: 1/1 passing ✅
evmora-aptos-vm:    1/1 passing ✅
evmora-quorlin-vm:  1/1 passing ✅
═══════════════════════════════════════
Total:             24/24 passing ✅
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p evmora-core

# With output
cargo test --workspace -- --nocapture

# Single test
cargo test test_memory_gas_expansion
```

---

## Performance

### Gas Efficiency Comparison

| Contract | Solidity | Quorlin | Improvement |
|----------|----------|---------|-------------|
| Counter Deploy | 65,598 gas | 60,951 gas | **-7.1%** ✅ |
| ERC20 Transfer | ~45,000 gas | ~45,000 gas | 0% |

### Memory Safety

- **DoS Protection:** 128 MB hard limit ✅
- **Yellow Paper Compliance:** Gas formula `(size² / 512) + (3 * size)` ✅
- **Out-of-Gas Detection:** Automatic with accurate metering ✅

---

## API Reference

### Quick API Overview

#### Compiler API

```rust
use evmora_compiler::{Compiler, CompileOpts};

let compiler = Compiler::new();
let opts = CompileOpts {
    language: Some("quorlin".to_string()),
    target: None,
    deterministic: true,
};
let artifact = compiler.compile_file("Contract.ql", opts)?;
```

#### Runtime API

```rust
use evmora_runtime::{EvmClient, Transaction};

let mut client = EvmClient::new("evmora.toml")?;
let tx = Transaction::create(bytecode, vec![], 1_000_000);
let result = client.execute(tx).await?;
```

#### Storage API

```rust
use primitive_types::H256;

let value = client.get_storage_at(contract_address, H256::zero())?;
```

**Full API documentation:** See [Documentations/](./Documentations/)

---

## Configuration

### Runtime Configuration (evmora.toml)

```toml
[runtime]
chain_id = 1337
block_gas_limit = 30000000
base_fee = 1000000000  # 1 gwei

[storage]
backend = "memory"  # Options: memory, rocksdb (planned)

[execution]
parallel_workers = 8
enable_tracing = true
max_call_depth = 1024
memory_limit = 134217728  # 128 MB
```

---

## Development

### Adding Custom Language Support

Implement the `CompilerFrontend` trait:

```rust
use evmora_compiler::frontends::CompilerFrontend;

pub struct MyLanguageFrontend;

impl CompilerFrontend for MyLanguageFrontend {
    fn name(&self) -> &str { "mylang" }
    fn extension(&self) -> &str { "ml" }
    
    fn compile_to_ir(&self, source: &str, target: Option<&str>) 
        -> anyhow::Result<IrProgram> {
        // Your compiler logic here
    }
}
```

### Custom Storage Backend

```rust
use evmora_plugins::StorageBackend;

pub struct MyStorage;

impl StorageBackend for MyStorage {
    fn get(&self, address: &H160, key: &H256) -> Option<U256> {
        // Your storage logic
    }
    
    fn set(&mut self, address: &H160, key: &H256, value: U256) {
        // Your storage logic
    }
}
```

---

## Troubleshooting

### Common Issues

**Quorlin: "Unexpected token {" error**
```python
# ❌ Wrong - missing self. prefix
fn increment() {
    count += 1;
}

# ✅ Correct - requires self. for state variables
fn increment() {
    self.count += 1;
}
```

**solc not found**
```bash
# Install Solidity compiler
# Ubuntu
sudo apt-get install solc

# macOS
brew install solidity
```

**Out of gas error**
```rust
// Increase gas limit
let tx = Transaction::create(bytecode, vec![], 2_000_000); // Increased
```

---

## Roadmap

### ✅ Completed (Current - Alpha)

- [x] Core EVM implementation with 100+ opcodes
- [x] Multi-language compiler (Solidity, Vyper, Quorlin, Move)
- [x] Yellow Paper-compliant gas metering
- [x] Thread-safe storage backend
- [x] Multi-VM architecture (4 VMs)
- [x] 24/24 core tests passing
- [x] DoS protection and security hardening

### 🚧 In Progress (Q1 2025)

- [ ] Ethereum Test Suite integration (target: 90%+ pass rate)
- [ ] Performance benchmarking vs Revm
- [ ] Quorlin parser improvements
- [ ] Parallel execution optimization
- [ ] Event log implementation

### 📅 Planned (Q2-Q3 2025)

- [ ] Cross-chain bridge completion
- [ ] RocksDB persistent storage
- [ ] JSON-RPC server
- [ ] Precompile contracts
- [ ] Security audit
- [ ] Production beta release

---

## Contributing

We welcome contributions! See our [contributing guidelines](CONTRIBUTING.md).

**Areas where we need help:**
- 🧪 Ethereum Test Suite integration
- ⚡ Performance optimization
- 🌐 New language frontends
- 📚 Documentation improvements
- 🛡️ Security reviews

---

## License

This project is licensed under the **MIT License** - see [LICENSE](LICENSE) for details.

---

## Acknowledgments

### Links

- **Website:** [https://evmora.xyz](https://evmora.xyz)
- **GitHub:** [https://github.com/EmekaIwuagwu/evmora-evm](https://github.com/EmekaIwuagwu/evmora-evm)
- **Documentation:** [./Documentations](./Documentations)
- **Demos:** [Quorlin](./QuorlinTestDemo) | [Solidity](./SolidityTestDemo)

### Credits

- **Ethereum Yellow Paper** - Specification reference
- **Revm** - Performance inspiration
- **Rust Community** - Outstanding tooling

---

## Project Status

**Current Phase:** Alpha  
**Grant Ready:** ✅ Yes - Core technology proven  
**Production Ready:** ⏳ Q3 2025 (after test suite integration)

**What Works:**
- ✅ Core EVM execution
- ✅ Multi-language compilation
- ✅ Contract deployment & execution
- ✅ Storage and gas metering
- ✅ Multi-VM support

**In Development:**
- ⏳ Ethereum test compliance
- ⏳ Performance optimization
- ⏳ Cross-chain features

---

**Built with discipline. Tested with rigor. Documented with honesty.** 🚀

*Last Updated: 2025-12-14*  
*Version: 0.1.0-alpha*
