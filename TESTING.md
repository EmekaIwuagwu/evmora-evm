# Testing Guide for Evmora EVM

This document outlines the strategy for performing full validation and testing of the Evmora EVM.

## 1. Overview
Evmora is currently an **execution engine library** and a **compiler toolchain**. It is designed to be embedded in a node or run locally.

**Do you need Digital Ocean?**
**No.** At this stage, Evmora is a local runtime engine (like the EVM inside Geth, but standalone). You do not need to deploy to a remote server (Digital Ocean, AWS) to verify correctness. All tests run locally on your machine using the CPU/Memory. Deployment to a cloud provider is only necessary if you are running a public-facing JSON-RPC node that others need to connect to (which would wrap this engine).

## 2. Automated Testing Suite
The primary way to validate the system is the Rust test suite.

### Running All Tests
```bash
cargo test --workspace
```

### Test Categories
| Scope | Command | Description |
|-------|---------|-------------|
| **Unit Tests** | `cargo test --lib` | Tests individual components like Stack, Memory, and Opcodes in isolation. |
| **Integration** | `cargo test --test e2e_multilang` | Compiles contracts, deploys them to an ephemeral EVM, and executes functions. |
| **Parallelism** | `cargo test --test parallel_exec` | Verifies the threaded execution engine handles batch loads correctly. |

---

## 3. Manual Validation (CLI & Data Flow)

You can manually simulate the "Developer Flow" (Compile -> Deploy -> Run) using the provided CLI tools and examples.

### Step 1: Compilation (The Request)
**Input**: A source file (e.g., `tests/fixtures/sol/Counter.sol`).
```solidity
contract Counter {
    uint256 count;
    function increment() { count += 1; }
}
```

**Command**:
```bash
cargo run --release --bin evmora-compiler -- compile ./tests/fixtures/sol/Counter.sol --lang sol --out debug_artifacts
```

**Output (The Response)**:
The compiler generates artifacts in `debug_artifacts/Counter/sol/`:
*   `bytecode.bin`: The machine code for the EVM.
*   `build-info.json`: Metadata about the build.

### Step 2: Execution (The Request)
Since there is no public JSON-RPC yet, "Requests" are constructed as Rust `Transaction` structs in the runtime examples.

**Scenario**: Deploying the contract compiled above.

**Internal Request Structure (`Transaction`)**:
```rust
Transaction {
    from: 0xAlice...,
    to: None, // None means Contract Creation
    value: 0,
    data: [0x60, 0x80, ...], // The content of bytecode.bin from Step 1
    gas_limit: 1_000_000,
    ...
}
```

**execution via Example**:
You can run a simulated transaction using the basic contract example:
```bash
cargo run -p evmora-runtime --example basic_contract
```

**Internal Response Structure (`ExecutionResult`)**:
```rust
ExecutionResult {
    success: true,
    return_data: [...], // The deployed contract address code
    gas_used: 24500,
    contract_address: Some(0x1234...),
    execution_time: 50µs
}
```

---

## 4. End-to-End Trace Example

If you want to trace a full execution logically without running code:

1.  **User compiles code**:
    *   *Request*: `evmora-compiler compile Token.ql`
    *   *Response*: `bytecode.bin` (contains ops: `PUSH1`, `MSTORE`, `SSTORE`...)

2.  **User sends transaction** (simulated via `EvmClient`):
    *   *Request*: `Transaction::call(to: TokenAddr, data: Selector + Args)`
    *   *System Action*:
        *   `parallel_executor` picks up tx.
        *   `evmora-core` initializes a Stack and Memory.
        *   `executor` loops through opcodes.
        *   Storage is updated in `InMemoryStorage`.
    *   *Response*: `ExecutionResult { success: true, return_data: 000...01 }`

## 5. Continuous Integration (CI)
For a production setup, instead of Digital Ocean, set up a GitHub Action:

```yaml
name: Evmora CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --workspace --release
```
