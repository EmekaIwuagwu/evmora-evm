# Quick Reference: Multi-Chain Deployment

## 🚀 Quick Start

### Deploy to EVM (Ethereum/Solidity)
```bash
cargo run --bin deploy evm --contract storage --gas-limit 1000000 --gas-price 20
```

### Deploy to Solana
```bash
cargo run --bin deploy solana --program token --lamports 100000
```

### Deploy to Polkadot
```bash
cargo run --bin deploy polkadot --contract flipper --weight 1000000000
```

### Deploy to Aptos
```bash
cargo run --bin deploy aptos --module coin --gas-limit 10000 --gas-price 100
```

### Deploy to Quorlin
```bash
cargo run --bin deploy quorlin --contract counter --units 50000 --gas-price 1
```

### Deploy to ALL Platforms
```bash
cargo run --bin deploy all --verbose
```

### Estimate Costs
```bash
cargo run --bin deploy estimate --platform all
```

## 📊 Gas Costs Quick Reference

| Platform | Unit | Example Cost | Native Token |
|----------|------|--------------|--------------|
| **EVM** | Gas | 74,000 @ 20 gwei | 0.00148 ETH |
| **Solana** | Lamports | 701,000 | 0.000701 SOL |
| **Polkadot** | Weight | 515,000,000 | Variable |
| **Aptos** | Gas Units | 2,500 @ 100 | 250,000 Octas |
| **Quorlin** | Exec Units | 1,200 @ 1 gwei | 0.0000012 ETH |

## 🧪 Testing

### Run All Tests
```bash
cargo test
```

### Run VM Tests
```bash
cargo test --test test_all_vms
```

### Run Deployment Tests
```bash
cargo test --test test_deployment
```

### Run Specific Platform Tests
```bash
cargo test --test test_deployment test_evm_
cargo test --test test_deployment test_solana_
cargo test --test test_deployment test_polkadot_
cargo test --test test_deployment test_aptos_
cargo test --test test_deployment test_quorlin_
```

### Run Cost Comparison
```bash
cargo test --test test_deployment test_deployment_cost_comparison -- --nocapture
```

## 📁 File Locations

### VM Implementations
- `crates/evmora-core/src/evm/` - EVM (Solidity)
- `crates/evmora-solana-vm/` - Solana BPF
- `crates/evmora-polkadot-vm/` - Polkadot WASM
- `crates/evmora-aptos-vm/` - Aptos Move
- `crates/evmora-quorlin-vm/` - Quorlin Native

### Deployment System
- `crates/evmora-runtime/src/deployment.rs` - Deployers
- `crates/evmora-runtime/src/contracts.rs` - Example contracts
- `tests/test_deployment.rs` - Tests
- `src/bin/deploy.rs` - CLI tool

### Documentation
- `VM_IMPLEMENTATION_COMPLETE.md` - VM docs
- `DEPLOYMENT_SYSTEM.md` - Deployment guide
- `COMPLETE_IMPLEMENTATION_SUMMARY.md` - Full summary
- `QUICK_REFERENCE.md` - This file

## 🔧 Available Contracts

### EVM
- `storage` - Simple storage contract
- `token` - ERC20-like token

### Solana
- `token` - SPL-like token program
- `counter` - Counter program

### Polkadot
- `flipper` - ink! flipper contract
- `storage` - Storage contract

### Aptos
- `coin` - Simple coin module
- `counter` - Counter module

### Quorlin
- `counter` - Counter contract
- `token` - Token contract

## ⚡ Common Tasks

### Check if VMs compile
```bash
cargo check --package evmora-quorlin-vm
cargo check --package evmora-solana-vm
cargo check --package evmora-polkadot-vm
cargo check --package evmora-aptos-vm
cargo check --package evmora-core
```

### Build release version
```bash
cargo build --release
```

### Run specific deployment test
```bash
cargo test --test test_deployment test_evm_simple_storage_deployment -- --nocapture
```

### View deployment costs
```bash
cargo run --bin deploy estimate --platform evm
cargo run --bin deploy estimate --platform solana
cargo run --bin deploy estimate --platform polkadot
cargo run --bin deploy estimate --platform aptos
cargo run --bin deploy estimate --platform quorlin
```

## 📈 Status Check

### Verify All Systems
```bash
# Check compilation
cargo check --all

# Run all tests
cargo test --all

# Build CLI tool
cargo build --bin deploy

# Test deployment to all platforms
cargo run --bin deploy all
```

## 🎯 Key Features

✅ **5 Platform Support** - EVM, Solana, Polkadot, Aptos, Quorlin  
✅ **Gas Fee Checking** - Pre-deployment validation  
✅ **Cost Estimation** - Accurate gas/fee calculation  
✅ **11 Example Contracts** - Ready-to-deploy  
✅ **30+ Tests** - Comprehensive coverage  
✅ **CLI Tool** - User-friendly interface  
✅ **Full Documentation** - Complete guides  

## 🚨 Troubleshooting

### Build fails
```bash
cargo clean
cargo build
```

### Tests fail
```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

### Deployment fails
- Check gas limit is sufficient
- Verify contract type is correct
- Ensure deployer address is valid

## 📞 Support

For issues or questions:
1. Check `DEPLOYMENT_SYSTEM.md` for detailed usage
2. Check `VM_IMPLEMENTATION_COMPLETE.md` for VM details
3. Run tests to verify system status
4. Review test output for specific errors

## ✅ Quick Verification

```bash
# Verify everything works
cargo test --test test_deployment test_all_platforms_deploy_successfully -- --nocapture
```

Expected output:
```
🚀 DEPLOYING TO ALL PLATFORMS
================================================================================
✅ EVM Deployment: SUCCESS
✅ Solana Deployment: SUCCESS
✅ Polkadot Deployment: SUCCESS
✅ Aptos Deployment: SUCCESS
✅ Quorlin Deployment: SUCCESS

📊 Results: 5/5 platforms deployed successfully
================================================================================
```

---

**Last Updated:** December 11, 2025  
**Status:** ✅ All Systems Operational  
**Version:** 1.0.0
