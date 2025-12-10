# Evmora EVM

Evmora is a next-generation Ethereum Virtual Machine (EVM) written in Rust, engineered for **performance**, **modularity**, and **multi-language support**. It provides a robust execution environment for smart contracts with sub-millisecond latency, parallel execution capabilities, and native bridging.

## 🌟 Key Functional Highlights

### 1. High-Performance Execution Core
At the heart of Evmora is `evmora-core`, a highly optimized implementation of the EVM specification.
*   **Stack**: A fixed-size (1024) high-speed stack for `U256` operations, with rigorous overflow/underflow protection (`crates/evmora-core/src/evm/stack.rs`).
*   **Memory**: Dynamic, gas-metered memory utilizing direct byte manipulation for `MLOAD`/`MSTORE` operations (`crates/evmora-core/src/evm/memory.rs`).
*   **Executor**: A streamlined opcode dispatch loop that integrates pluggable storage and gas calculation mechanisms.

### 2. Native Multi-Language Compiler
Evmora includes a built-in compiler framework (`evmora-compiler`) that transpiles various high-level languages into a shared Intermediate Representation (IR) before generating EVM bytecode.

**Supported Frontends:**
*   **Solidity (`.sol`)**: Full support for standard Ethereum smart contracts.
*   **Quorlin (`.ql`)**: Native support for the [Quorlin](https://quorlin.dev) language.
    *   **Generic Parsing**: Dynamically parses functions, decorators (e.g., `@external`), and Pythonic syntax to generate compliant bytecode.
*   **Vyper (`.vy`)**: Support for Python-ic smart contract logic.
*   **Move (`.move`)**: Experimental support for Move modules.
*   **Extensible Architecture**: New languages can be added by simply implementing the `CompilerFrontend` trait (`crates/evmora-compiler/src/frontends/traits.rs`).

### 3. Integrated Cross-Chain Bridge
The system features a native bridging module (`evmora-bridge`) for seamless asset interoperability.
*   **Adapters**: Includes pre-built adapters for **Ethereum**, **Polygon**, and generic EVM chains.
*   **Asset Support**: Native structs for **ERC-20**, **ERC-721**, and **ERC-1155** token standards.
*   **Security**: Integrated Merkle Proof verification for validating cross-chain messages.

### 4. Parallel Execution Engine
Evmora utilizes `Rayon` to implement an optimistic parallel execution model (`evmora-runtime/src/parallel.rs`), allowing non-conflicting transactions to be processed simultaneously on multi-core systems.

## 🏗️ Architecture & Crates

| Crate | Role | Key Components |
|-------|------|----------------|
| **`evmora-core`** | **The Engine** | `Stack`, `Memory`, `Opcode` Execution, `Storage` traits. |
| **`evmora-runtime`** | **The Controller** | `EvmClient`, `ParallelExecutor`, Transaction Pool. |
| **`evmora-compiler`** | **The Translator** | `Compiler` struct, `IrProgram`, `QuorlinFrontend`, `SolidityFrontend`. |
| **`evmora-bridge`** | **The Connector** | `BridgeManager`, `ChainAdapter`, `Erc20Token`. |
| **`evmora-plugins`** | **The Interfaces** | `GasCalculator`, `StorageBackend`, `EvmPlugin` traits. |
| **`evmora-utils`** | **The Toolbox** | Crypto primitives (`Keccak256`, `Secp256k1`), Config parsing. |

## � Getting Started

### Prerequisites
*   Rust (latest stable)
*   Cargo

### Build
Compile the entire workspace in release mode for maximum performance:
```bash
cargo build --release --workspace
```

### Testing & Validation
Run the full test suite to verify the Core, Compiler, and Bridge:
```bash
cargo test --workspace
```

### Running Examples

**1. Multi-Language Compilation**
Compile Quorlin, Solidity, Vyper, and Move contracts to EVM bytecode:
```bash
cargo run -p evmora-compiler --example multilang_compile
```

**2. Basic Contract Execution**
Deploy and run a raw bytecode contract:
```bash
cargo run -p evmora-runtime --example basic_contract
```

**3. Cross-Chain Bridging**
Simulate a bridge transfer logic (requires setup):
```bash
cargo run -p evmora-bridge --example cross_chain_bridge
```

## 📜 License
This project is licensed under the MIT License.

---
*Built with ❤️ in Rust for the future of decentralized execution.*
