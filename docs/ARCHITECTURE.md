# Evmora Architecture

## Overview

Evmora is designed as a modular, high-performance EVM implementation. It separates the core execution logic from external concerns like storage synchronization, gas estimation strategies, and cross-chain bridging.

## Core Components

### evmora-core
- **Stack**: Fixed-size 1024 element stack of U256.
- **Memory**: Dynamic byte array with gas metering (quadratic cost).
- **Executor**: Main loop processing opcodes, using a plugin-based GasCalculator and StorageBackend.
- **Opcodes**: Implementation of EVM instructions.

### evmora-runtime
- **EvmClient**: High-level API for transaction execution.
- **ParallelExecutor**: Optimistic parallel execution engine using Rayon.
- **State**: Manages the world state (accounts, storage).

### evmora-plugins
Defines traits for extensibility:
- `GasCalculator`: Custom gas logic.
- `StorageBackend`: Pluggable storage (InMemory, RocksDB, etc.).
- `BridgeAdapter`: Cross-chain communication.

### evmora-bridge
Implements bridging logic using adapters (Ethereum, Polygon, etc.) and token standards (ERC20, ERC721).

## Performance

- **Parallelism**: Transactions are executed in parallel when possible.
- **Optimization**: Gas calculation is decoupled to allow for static analysis optimization.

## Security

- **Safe Rust**: Usage of Rust's ownership model to prevent memory safety issues.
- **Validation**: Strict opcode validation and stack bounds checking.
