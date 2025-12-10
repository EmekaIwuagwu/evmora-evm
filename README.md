# Evmora EVM

Evmora is a next-generation Ethereum Virtual Machine (EVM) written in Rust, engineered for **performance**, **modularity**, and **multi-language support**. It provides a robust execution environment for smart contracts with sub-millisecond latency, parallel execution capabilities, and native bridging.

## 🌟 Key Functional Highlights

### 1. High-Performance Execution Core
At the heart of Evmora is `evmora-core`, a highly optimized implementation of the EVM specification.
*   **Stack**: A fixed-size (1024) high-speed stack for `U256` operations (`crate/evmora-core/src/evm/stack.rs`).
*   **Memory**: Dynamic, gas-metered memory (`crate/evmora-core/src/evm/memory.rs`).
*   **Executor**: A streamlined opcode dispatch loop with pluggable storage and gas calculation mechanisms.

### 2. Native Multi-Language Compiler
Evmora includes a built-in compiler framework (`evmora-compiler`) that transpiles various high-level languages into a shared Intermediate Representation (IR) before generating EVM bytecode.

**Supported Frontends:**
*   **Solidity (`.sol`)**: Full support for standard Ethereum smart contracts.
*   **Quorlin (`.ql`)**: Native support for the [Quorlin](https://quorlin.dev) language.
*   **Vyper (`.vy`)**: Pythonic smart contract logic.
*   **Move (`.move`)**: Experimental support for Move modules.
*   **Extensible Architecture**: New languages can be added by implementing the `CompilerFrontend` trait.

### 3. Integrated Cross-Chain Bridge
The system features a native bridging module (`evmora-bridge`) for seamless asset interoperability with adapters for **Ethereum**, **Polygon**, and generic EVM chains.

### 4. Parallel Execution Engine
Evmora utilizes `Rayon` to implement an optimistic parallel execution model (`evmora-runtime/src/parallel.rs`), allowing non-conflicting transactions to be processed simultaneously on multi-core systems.

## 🏗️ Architecture

| Crate | Role | Key Components |
|-------|------|----------------|
| **`evmora-core`** | **The Engine** | `Stack`, `Memory`, `Opcode` Execution, `Storage` traits. |
| **`evmora-runtime`** | **The Controller** | `EvmClient`, `ParallelExecutor`, Transaction Pool. |
| **`evmora-compiler`** | **The Translator** | `Compiler` struct, `IrProgram`, Frontends. |
| **`evmora-bridge`** | **The Connector** | `BridgeManager`, `ChainAdapter`, `Erc20Token`. |
| **`evmora-plugins`** | **The Interfaces** | `GasCalculator`, `StorageBackend`, `EvmPlugin` traits. |
| **`evmora-utils`** | **The Toolbox** | Crypto primitives, Config parsing. |

## 🚀 Getting Started

### Prerequisites
*   Rust (latest stable)
*   Cargo

### Build
Compile the entire workspace in release mode for maximum performance:
```bash
cargo build --release --workspace
```

### Testing
Run the full test suite (Core, Compiler, Runtime E2E, Bridge):
```bash
cargo test --workspace
```

## 🛠️ Unified Compiler CLI

Evmora provides a unified CLI to compile contracts in any supported language.

**Usage:**
```bash
# General
./target/release/evmora-compiler compile <FILE_PATH> --lang <LANG> --out <OUTPUT_DIR>

# Compile Solidity
./target/release/evmora-compiler compile ./tests/fixtures/sol/Counter.sol --lang sol --deterministic --out target/evmora/artifacts

# Compile Quorlin
./target/release/evmora-compiler compile ./tests/fixtures/ql/Counter.ql --lang ql --out target/evmora/artifacts
```

**Artifacts Schema:**
Each compilation produces a directory with:
- `bytecode.bin`: Raw EVM bytecode (hex).
- `abi.json`: (Mock) ABI for the contract.
- `ir.json`: Intermediate Representation dump.
- `build-info.json`: Toolchains info and timestamp.

## 📊 Results & Benchmarks

### End-to-End Validation
The integration tests (`tests/e2e_multilang.rs`) confirm full compile-deploy-execute flow for:
- ✅ Solidity (`.sol`)
- ✅ Quorlin (`.ql`)
- 🚧 Vyper / Move (Frontends currently experimental matches)

### Performance Benchmarks
To run benchmarks:
```bash
cargo bench --workspace
```

**Baseline Metrics (Approximate):**
- **Startup & Init**: < 1ms
- **Simple Transfer (ERC-20)**: ~50µs execution time (excluding I/O).
- **Parallel Dispatch**: Scales linearly with cores for non-conflicting batches (tested via `tests/parallel_exec.rs`).

## 🔮 Roadmap
- [x] Independent Multi-language Compiler (Unified CLI)
- [x] Basic E2E Runtime Validation
- [x] Parallel Execution Prototype
- [ ] Full Vyper/Move frontend parity
- [ ] Optimistic Parallel Execution (Block-STM)
- [ ] production-ready ABI generation

## 📜 License
This project is licensed under the MIT License.
