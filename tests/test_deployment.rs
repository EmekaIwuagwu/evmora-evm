// Comprehensive Deployment Tests with Gas Fee Verification

use evmora_runtime::deployment::*;
use evmora_runtime::contracts::*;
use primitive_types::U256;

#[cfg(test)]
mod deployment_tests {
    use super::*;
    
    // ========================================================================
    // EVM (Solidity) Deployment Tests
    // ========================================================================
    
    #[test]
    fn test_evm_simple_storage_deployment() {
        let mut deployer = EvmDeployer::new();
        let bytecode = solidity::simple_storage_bytecode();
        
        let config = DeploymentConfig {
            gas_limit: 1_000_000,
            gas_price: U256::from(20_000_000_000u64), // 20 gwei
            value: U256::zero(),
            deployer: vec![1; 20],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("✅ EVM Simple Storage Deployment:");
        println!("   Contract Address: 0x{}", hex::encode(&result.contract_address));
        println!("   Gas Used: {}", result.gas_used);
        println!("   Gas Cost: {} wei", result.gas_cost);
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        println!("   Transaction Hash: 0x{}", hex::encode(&result.transaction_hash));
        
        assert!(result.success);
        assert_eq!(result.contract_address.len(), 20);
        assert!(result.gas_used > 0);
        assert!(result.gas_cost > U256::zero());
    }
    
    #[test]
    fn test_evm_token_deployment() {
        let mut deployer = EvmDeployer::new();
        let bytecode = solidity::simple_token_bytecode();
        
        let config = DeploymentConfig {
            gas_limit: 2_000_000,
            gas_price: U256::from(50_000_000_000u64), // 50 gwei
            value: U256::zero(),
            deployer: vec![2; 20],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ EVM Token Deployment:");
        println!("   Contract Address: 0x{}", hex::encode(&result.contract_address));
        println!("   Gas Used: {}", result.gas_used);
        println!("   Gas Cost: {} wei ({} ETH)", 
            result.gas_cost, 
            result.gas_cost.as_u128() as f64 / 1e18
        );
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        
        assert!(result.success);
        assert!(result.gas_used < 2_000_000);
    }
    
    #[test]
    fn test_evm_insufficient_gas() {
        let mut deployer = EvmDeployer::new();
        let bytecode = solidity::simple_storage_bytecode();
        
        let config = DeploymentConfig {
            gas_limit: 1000, // Intentionally too low
            gas_price: U256::from(20_000_000_000u64),
            value: U256::zero(),
            deployer: vec![3; 20],
        };
        
        let result = deployer.deploy(&bytecode, config);
        
        println!("\n✅ EVM Insufficient Gas Test:");
        println!("   Result: {:?}", result);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Insufficient gas"));
    }
    
    #[test]
    fn test_evm_gas_estimation() {
        let deployer = EvmDeployer::new();
        let bytecode = solidity::simple_storage_bytecode();
        
        let estimated = deployer.estimate_gas(&bytecode).unwrap();
        
        println!("\n✅ EVM Gas Estimation:");
        println!("   Bytecode Size: {} bytes", bytecode.len());
        println!("   Estimated Gas: {}", estimated);
        println!("   Base Cost: 21000 (tx) + 32000 (create)");
        println!("   Storage Cost: {} * 200 = {}", bytecode.len(), bytecode.len() * 200);
        
        assert!(estimated > 53000); // At least base + create
    }
    
    // ========================================================================
    // Solana Deployment Tests
    // ========================================================================
    
    #[test]
    fn test_solana_token_deployment() {
        let mut deployer = SolanaDeployer::new();
        let bytecode = solana::token_program();
        
        let config = DeploymentConfig {
            gas_limit: 100_000, // Lamports
            gas_price: U256::one(),
            value: U256::zero(),
            deployer: vec![1; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Solana Token Deployment:");
        println!("   Program ID: {}", hex::encode(&result.contract_address));
        println!("   Lamports Used: {}", result.gas_used);
        println!("   Cost: {} lamports ({} SOL)", 
            result.gas_cost, 
            result.gas_cost.as_u128() as f64 / 1e9
        );
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        
        assert!(result.success);
        assert_eq!(result.contract_address.len(), 32);
    }
    
    #[test]
    fn test_solana_counter_deployment() {
        let mut deployer = SolanaDeployer::new();
        let bytecode = solana::counter_program();
        
        let config = DeploymentConfig {
            gas_limit: 50_000,
            gas_price: U256::one(),
            value: U256::zero(),
            deployer: vec![2; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Solana Counter Deployment:");
        println!("   Program ID: {}", hex::encode(&result.contract_address));
        println!("   Lamports Used: {}", result.gas_used);
        
        assert!(result.success);
    }
    
    #[test]
    fn test_solana_rent_calculation() {
        let deployer = SolanaDeployer::new();
        let bytecode = solana::token_program();
        
        let estimated = deployer.estimate_gas(&bytecode).unwrap();
        
        println!("\n✅ Solana Rent Calculation:");
        println!("   Program Size: {} bytes", bytecode.len());
        println!("   Rent-Exempt Minimum: {} lamports", estimated - 5000);
        println!("   Deployment Fee: 5000 lamports");
        println!("   Total Required: {} lamports", estimated);
        
        assert!(estimated > bytecode.len() as u64);
    }
    
    // ========================================================================
    // Polkadot/Substrate Deployment Tests
    // ========================================================================
    
    #[test]
    fn test_polkadot_flipper_deployment() {
        let mut deployer = PolkadotDeployer::new();
        let bytecode = polkadot::flipper_contract();
        
        let config = DeploymentConfig {
            gas_limit: 1_000_000_000, // Weight units
            gas_price: U256::from(1_000_000u64),
            value: U256::from(1_000_000_000_000u64), // 1 unit
            deployer: vec![1; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Polkadot Flipper Deployment:");
        println!("   Contract Address: {}", hex::encode(&result.contract_address));
        println!("   Weight Used: {}", result.gas_used);
        println!("   Cost: {} tokens", result.gas_cost);
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        
        assert!(result.success);
        assert_eq!(result.contract_address.len(), 32);
    }
    
    #[test]
    fn test_polkadot_storage_deployment() {
        let mut deployer = PolkadotDeployer::new();
        let bytecode = polkadot::storage_contract();
        
        let config = DeploymentConfig {
            gas_limit: 800_000_000,
            gas_price: U256::from(1_000_000u64),
            value: U256::zero(),
            deployer: vec![2; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Polkadot Storage Deployment:");
        println!("   Contract Address: {}", hex::encode(&result.contract_address));
        println!("   Weight Used: {}", result.gas_used);
        
        assert!(result.success);
    }
    
    #[test]
    fn test_polkadot_weight_calculation() {
        let deployer = PolkadotDeployer::new();
        let bytecode = polkadot::flipper_contract();
        
        let estimated = deployer.estimate_gas(&bytecode).unwrap();
        
        println!("\n✅ Polkadot Weight Calculation:");
        println!("   WASM Size: {} bytes", bytecode.len());
        println!("   Base Weight: 500,000,000");
        println!("   Storage Weight: {} * 100,000", bytecode.len());
        println!("   Compile Weight: {} * 50,000", bytecode.len());
        println!("   Total Weight: {}", estimated);
        
        assert!(estimated > 500_000_000);
    }
    
    // ========================================================================
    // Aptos Move Deployment Tests
    // ========================================================================
    
    #[test]
    fn test_aptos_coin_deployment() {
        let mut deployer = AptosDeployer::new();
        let bytecode = aptos::simple_coin_module();
        
        let config = DeploymentConfig {
            gas_limit: 10_000,
            gas_price: U256::from(100u64), // Gas unit price
            value: U256::zero(),
            deployer: vec![1; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Aptos Coin Module Deployment:");
        println!("   Module Address: {}", hex::encode(&result.contract_address));
        println!("   Gas Units Used: {}", result.gas_used);
        println!("   Cost: {} Octas", result.gas_cost);
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        
        assert!(result.success);
        assert_eq!(result.contract_address.len(), 32);
    }
    
    #[test]
    fn test_aptos_counter_deployment() {
        let mut deployer = AptosDeployer::new();
        let bytecode = aptos::counter_module();
        
        let config = DeploymentConfig {
            gas_limit: 5_000,
            gas_price: U256::from(100u64),
            value: U256::zero(),
            deployer: vec![2; 32],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Aptos Counter Module Deployment:");
        println!("   Module Address: {}", hex::encode(&result.contract_address));
        println!("   Gas Units Used: {}", result.gas_used);
        
        assert!(result.success);
    }
    
    #[test]
    fn test_aptos_gas_calculation() {
        let deployer = AptosDeployer::new();
        let bytecode = aptos::simple_coin_module();
        
        let estimated = deployer.estimate_gas(&bytecode).unwrap();
        
        println!("\n✅ Aptos Gas Calculation:");
        println!("   Module Size: {} bytes", bytecode.len());
        println!("   Base Gas: 1000");
        println!("   Bytecode Gas: {} * 10", bytecode.len());
        println!("   Verification Gas: 500");
        println!("   Total Gas Units: {}", estimated);
        
        assert!(estimated > 1500);
    }
    
    // ========================================================================
    // Quorlin Native Deployment Tests
    // ========================================================================
    
    #[test]
    fn test_quorlin_counter_deployment() {
        let mut deployer = QuorlinDeployer::new();
        let bytecode = quorlin::counter_bytecode();
        
        let config = DeploymentConfig {
            gas_limit: 50_000,
            gas_price: U256::from(1_000_000_000u64), // 1 gwei
            value: U256::zero(),
            deployer: vec![1; 20],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Quorlin Counter Deployment:");
        println!("   Contract ID: 0x{}", hex::encode(&result.contract_address));
        println!("   Execution Units: {}", result.gas_used);
        println!("   Cost: {} wei", result.gas_cost);
        println!("   Deployment Time: {}ms", result.deployment_time_ms);
        
        assert!(result.success);
        assert_eq!(result.contract_address.len(), 20);
    }
    
    #[test]
    fn test_quorlin_token_deployment() {
        let mut deployer = QuorlinDeployer::new();
        let bytecode = quorlin::token_bytecode();
        
        let config = DeploymentConfig {
            gas_limit: 100_000,
            gas_price: U256::from(2_000_000_000u64), // 2 gwei
            value: U256::zero(),
            deployer: vec![2; 20],
        };
        
        let result = deployer.deploy(&bytecode, config).unwrap();
        
        println!("\n✅ Quorlin Token Deployment:");
        println!("   Contract ID: 0x{}", hex::encode(&result.contract_address));
        println!("   Execution Units: {}", result.gas_used);
        
        assert!(result.success);
    }
    
    #[test]
    fn test_quorlin_execution_units() {
        let deployer = QuorlinDeployer::new();
        let bytecode = quorlin::counter_bytecode();
        
        let estimated = deployer.estimate_gas(&bytecode).unwrap();
        
        println!("\n✅ Quorlin Execution Units:");
        println!("   Bytecode Size: {} bytes", bytecode.len());
        println!("   Estimated Units: {}", estimated);
        println!("   Base Deployment: 1000 units");
        
        assert!(estimated > 1000);
    }
    
    // ========================================================================
    // Cross-Platform Comparison Tests
    // ========================================================================
    
    #[test]
    fn test_deployment_cost_comparison() {
        println!("\n📊 DEPLOYMENT COST COMPARISON");
        println!("=" .repeat(80));
        
        // EVM
        let evm_deployer = EvmDeployer::new();
        let evm_bytecode = solidity::simple_storage_bytecode();
        let evm_gas = evm_deployer.estimate_gas(&evm_bytecode).unwrap();
        let evm_cost = evm_deployer.calculate_deployment_cost(evm_gas, U256::from(20_000_000_000u64));
        println!("\n🔷 EVM (Solidity):");
        println!("   Gas: {} units", evm_gas);
        println!("   Cost: {} wei ({} ETH @ 20 gwei)", evm_cost, evm_cost.as_u128() as f64 / 1e18);
        
        // Solana
        let solana_deployer = SolanaDeployer::new();
        let solana_bytecode = solana::token_program();
        let solana_lamports = solana_deployer.estimate_gas(&solana_bytecode).unwrap();
        println!("\n🟣 Solana:");
        println!("   Lamports: {}", solana_lamports);
        println!("   Cost: {} SOL", solana_lamports as f64 / 1e9);
        
        // Polkadot
        let polkadot_deployer = PolkadotDeployer::new();
        let polkadot_bytecode = polkadot::flipper_contract();
        let polkadot_weight = polkadot_deployer.estimate_gas(&polkadot_bytecode).unwrap();
        println!("\n🔴 Polkadot/Substrate:");
        println!("   Weight: {} units", polkadot_weight);
        
        // Aptos
        let aptos_deployer = AptosDeployer::new();
        let aptos_bytecode = aptos::simple_coin_module();
        let aptos_gas = aptos_deployer.estimate_gas(&aptos_bytecode).unwrap();
        println!("\n⚫ Aptos:");
        println!("   Gas Units: {}", aptos_gas);
        println!("   Cost: {} Octas (@ 100 gas price)", aptos_gas * 100);
        
        // Quorlin
        let quorlin_deployer = QuorlinDeployer::new();
        let quorlin_bytecode = quorlin::counter_bytecode();
        let quorlin_units = quorlin_deployer.estimate_gas(&quorlin_bytecode).unwrap();
        println!("\n🟢 Quorlin:");
        println!("   Execution Units: {}", quorlin_units);
        
        println!("\n" + &"=".repeat(80));
    }
    
    #[test]
    fn test_all_platforms_deploy_successfully() {
        println!("\n🚀 DEPLOYING TO ALL PLATFORMS");
        println!("=" .repeat(80));
        
        let mut success_count = 0;
        let total_platforms = 5;
        
        // EVM
        let mut evm = EvmDeployer::new();
        if evm.deploy(&solidity::simple_storage_bytecode(), DeploymentConfig {
            gas_limit: 1_000_000,
            gas_price: U256::from(20_000_000_000u64),
            value: U256::zero(),
            deployer: vec![1; 20],
        }).is_ok() {
            println!("✅ EVM Deployment: SUCCESS");
            success_count += 1;
        }
        
        // Solana
        let mut solana = SolanaDeployer::new();
        if solana.deploy(&solana::token_program(), DeploymentConfig {
            gas_limit: 100_000,
            gas_price: U256::one(),
            value: U256::zero(),
            deployer: vec![1; 32],
        }).is_ok() {
            println!("✅ Solana Deployment: SUCCESS");
            success_count += 1;
        }
        
        // Polkadot
        let mut polkadot = PolkadotDeployer::new();
        if polkadot.deploy(&polkadot::flipper_contract(), DeploymentConfig {
            gas_limit: 1_000_000_000,
            gas_price: U256::from(1_000_000u64),
            value: U256::zero(),
            deployer: vec![1; 32],
        }).is_ok() {
            println!("✅ Polkadot Deployment: SUCCESS");
            success_count += 1;
        }
        
        // Aptos
        let mut aptos = AptosDeployer::new();
        if aptos.deploy(&aptos::simple_coin_module(), DeploymentConfig {
            gas_limit: 10_000,
            gas_price: U256::from(100u64),
            value: U256::zero(),
            deployer: vec![1; 32],
        }).is_ok() {
            println!("✅ Aptos Deployment: SUCCESS");
            success_count += 1;
        }
        
        // Quorlin
        let mut quorlin = QuorlinDeployer::new();
        if quorlin.deploy(&quorlin::counter_bytecode(), DeploymentConfig {
            gas_limit: 50_000,
            gas_price: U256::from(1_000_000_000u64),
            value: U256::zero(),
            deployer: vec![1; 20],
        }).is_ok() {
            println!("✅ Quorlin Deployment: SUCCESS");
            success_count += 1;
        }
        
        println!("\n📊 Results: {}/{} platforms deployed successfully", success_count, total_platforms);
        println!("=" .repeat(80));
        
        assert_eq!(success_count, total_platforms, "All platforms should deploy successfully");
    }
}
