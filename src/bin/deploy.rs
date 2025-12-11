// Multi-Chain Smart Contract Deployment CLI

use clap::{Parser, Subcommand};
use anyhow::Result;
use primitive_types::U256;
use evmora_runtime::deployment::*;
use evmora_runtime::contracts::*;

#[derive(Parser)]
#[command(name = "evmora-deploy")]
#[command(about = "Deploy smart contracts to multiple blockchains", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy to Ethereum/EVM
    Evm {
        /// Contract type: storage, token
        #[arg(short, long)]
        contract: String,
        
        /// Gas limit
        #[arg(short, long, default_value = "1000000")]
        gas_limit: u64,
        
        /// Gas price in gwei
        #[arg(short = 'p', long, default_value = "20")]
        gas_price: u64,
        
        /// Deployer address (hex)
        #[arg(short, long)]
        deployer: Option<String>,
    },
    
    /// Deploy to Solana
    Solana {
        /// Program type: token, counter
        #[arg(short, long)]
        program: String,
        
        /// Lamports limit
        #[arg(short, long, default_value = "100000")]
        lamports: u64,
        
        /// Deployer pubkey (hex)
        #[arg(short, long)]
        deployer: Option<String>,
    },
    
    /// Deploy to Polkadot/Substrate
    Polkadot {
        /// Contract type: flipper, storage
        #[arg(short, long)]
        contract: String,
        
        /// Weight limit
        #[arg(short, long, default_value = "1000000000")]
        weight: u64,
        
        /// Initial balance
        #[arg(short, long, default_value = "0")]
        balance: u128,
        
        /// Deployer address (hex)
        #[arg(short, long)]
        deployer: Option<String>,
    },
    
    /// Deploy to Aptos
    Aptos {
        /// Module type: coin, counter
        #[arg(short, long)]
        module: String,
        
        /// Gas units limit
        #[arg(short, long, default_value = "10000")]
        gas_limit: u64,
        
        /// Gas price
        #[arg(short = 'p', long, default_value = "100")]
        gas_price: u64,
        
        /// Deployer address (hex)
        #[arg(short, long)]
        deployer: Option<String>,
    },
    
    /// Deploy to Quorlin
    Quorlin {
        /// Contract type: counter, token
        #[arg(short, long)]
        contract: String,
        
        /// Execution units limit
        #[arg(short, long, default_value = "50000")]
        units: u64,
        
        /// Gas price in gwei
        #[arg(short = 'p', long, default_value = "1")]
        gas_price: u64,
        
        /// Deployer address (hex)
        #[arg(short, long)]
        deployer: Option<String>,
    },
    
    /// Deploy to all platforms
    All {
        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Estimate deployment costs
    Estimate {
        /// Platform: evm, solana, polkadot, aptos, quorlin, all
        #[arg(short, long)]
        platform: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Evm { contract, gas_limit, gas_price, deployer } => {
            deploy_evm(contract, gas_limit, gas_price, deployer)?;
        }
        Commands::Solana { program, lamports, deployer } => {
            deploy_solana(program, lamports, deployer)?;
        }
        Commands::Polkadot { contract, weight, balance, deployer } => {
            deploy_polkadot(contract, weight, balance, deployer)?;
        }
        Commands::Aptos { module, gas_limit, gas_price, deployer } => {
            deploy_aptos(module, gas_limit, gas_price, deployer)?;
        }
        Commands::Quorlin { contract, units, gas_price, deployer } => {
            deploy_quorlin(contract, units, gas_price, deployer)?;
        }
        Commands::All { verbose } => {
            deploy_all(verbose)?;
        }
        Commands::Estimate { platform } => {
            estimate_costs(platform)?;
        }
    }
    
    Ok(())
}

fn deploy_evm(contract: String, gas_limit: u64, gas_price: u64, deployer: Option<String>) -> Result<()> {
    println!("🔷 Deploying to EVM (Ethereum)...\n");
    
    let bytecode = match contract.as_str() {
        "storage" => solidity::simple_storage_bytecode(),
        "token" => solidity::simple_token_bytecode(),
        _ => {
            println!("❌ Unknown contract type. Use: storage, token");
            return Ok(());
        }
    };
    
    let deployer_addr = parse_deployer(deployer, 20);
    
    let mut deployer_instance = EvmDeployer::new();
    
    let config = DeploymentConfig {
        gas_limit,
        gas_price: U256::from(gas_price) * U256::from(1_000_000_000u64), // Convert gwei to wei
        value: U256::zero(),
        deployer: deployer_addr,
    };
    
    let result = deployer_instance.deploy(&bytecode, config)?;
    
    print_deployment_result("EVM", result);
    
    Ok(())
}

fn deploy_solana(program: String, lamports: u64, deployer: Option<String>) -> Result<()> {
    println!("🟣 Deploying to Solana...\n");
    
    let bytecode = match program.as_str() {
        "token" => solana::token_program(),
        "counter" => solana::counter_program(),
        _ => {
            println!("❌ Unknown program type. Use: token, counter");
            return Ok(());
        }
    };
    
    let deployer_addr = parse_deployer(deployer, 32);
    
    let mut deployer_instance = SolanaDeployer::new();
    
    let config = DeploymentConfig {
        gas_limit: lamports,
        gas_price: U256::one(),
        value: U256::zero(),
        deployer: deployer_addr,
    };
    
    let result = deployer_instance.deploy(&bytecode, config)?;
    
    print_deployment_result("Solana", result);
    
    Ok(())
}

fn deploy_polkadot(contract: String, weight: u64, balance: u128, deployer: Option<String>) -> Result<()> {
    println!("🔴 Deploying to Polkadot/Substrate...\n");
    
    let bytecode = match contract.as_str() {
        "flipper" => polkadot::flipper_contract(),
        "storage" => polkadot::storage_contract(),
        _ => {
            println!("❌ Unknown contract type. Use: flipper, storage");
            return Ok(());
        }
    };
    
    let deployer_addr = parse_deployer(deployer, 32);
    
    let mut deployer_instance = PolkadotDeployer::new();
    
    let config = DeploymentConfig {
        gas_limit: weight,
        gas_price: U256::from(1_000_000u64),
        value: U256::from(balance),
        deployer: deployer_addr,
    };
    
    let result = deployer_instance.deploy(&bytecode, config)?;
    
    print_deployment_result("Polkadot", result);
    
    Ok(())
}

fn deploy_aptos(module: String, gas_limit: u64, gas_price: u64, deployer: Option<String>) -> Result<()> {
    println!("⚫ Deploying to Aptos...\n");
    
    let bytecode = match module.as_str() {
        "coin" => aptos::simple_coin_module(),
        "counter" => aptos::counter_module(),
        _ => {
            println!("❌ Unknown module type. Use: coin, counter");
            return Ok(());
        }
    };
    
    let deployer_addr = parse_deployer(deployer, 32);
    
    let mut deployer_instance = AptosDeployer::new();
    
    let config = DeploymentConfig {
        gas_limit,
        gas_price: U256::from(gas_price),
        value: U256::zero(),
        deployer: deployer_addr,
    };
    
    let result = deployer_instance.deploy(&bytecode, config)?;
    
    print_deployment_result("Aptos", result);
    
    Ok(())
}

fn deploy_quorlin(contract: String, units: u64, gas_price: u64, deployer: Option<String>) -> Result<()> {
    println!("🟢 Deploying to Quorlin...\n");
    
    let bytecode = match contract.as_str() {
        "counter" => quorlin::counter_bytecode(),
        "token" => quorlin::token_bytecode(),
        _ => {
            println!("❌ Unknown contract type. Use: counter, token");
            return Ok(());
        }
    };
    
    let deployer_addr = parse_deployer(deployer, 20);
    
    let mut deployer_instance = QuorlinDeployer::new();
    
    let config = DeploymentConfig {
        gas_limit: units,
        gas_price: U256::from(gas_price) * U256::from(1_000_000_000u64), // Convert gwei to wei
        value: U256::zero(),
        deployer: deployer_addr,
    };
    
    let result = deployer_instance.deploy(&bytecode, config)?;
    
    print_deployment_result("Quorlin", result);
    
    Ok(())
}

fn deploy_all(verbose: bool) -> Result<()> {
    println!("🚀 Deploying to ALL platforms...\n");
    println!("=" .repeat(80));
    
    let mut results = Vec::new();
    
    // EVM
    let mut evm = EvmDeployer::new();
    match evm.deploy(&solidity::simple_storage_bytecode(), DeploymentConfig {
        gas_limit: 1_000_000,
        gas_price: U256::from(20_000_000_000u64),
        value: U256::zero(),
        deployer: vec![1; 20],
    }) {
        Ok(result) => {
            results.push(("EVM", true, result.gas_used, result.gas_cost));
            if verbose {
                print_deployment_result("EVM", result);
            } else {
                println!("✅ EVM: Deployed successfully");
            }
        }
        Err(e) => {
            results.push(("EVM", false, 0, U256::zero()));
            println!("❌ EVM: {}", e);
        }
    }
    
    // Solana
    let mut solana = SolanaDeployer::new();
    match solana.deploy(&solana::token_program(), DeploymentConfig {
        gas_limit: 100_000,
        gas_price: U256::one(),
        value: U256::zero(),
        deployer: vec![1; 32],
    }) {
        Ok(result) => {
            results.push(("Solana", true, result.gas_used, result.gas_cost));
            if verbose {
                print_deployment_result("Solana", result);
            } else {
                println!("✅ Solana: Deployed successfully");
            }
        }
        Err(e) => {
            results.push(("Solana", false, 0, U256::zero()));
            println!("❌ Solana: {}", e);
        }
    }
    
    // Polkadot
    let mut polkadot = PolkadotDeployer::new();
    match polkadot.deploy(&polkadot::flipper_contract(), DeploymentConfig {
        gas_limit: 1_000_000_000,
        gas_price: U256::from(1_000_000u64),
        value: U256::zero(),
        deployer: vec![1; 32],
    }) {
        Ok(result) => {
            results.push(("Polkadot", true, result.gas_used, result.gas_cost));
            if verbose {
                print_deployment_result("Polkadot", result);
            } else {
                println!("✅ Polkadot: Deployed successfully");
            }
        }
        Err(e) => {
            results.push(("Polkadot", false, 0, U256::zero()));
            println!("❌ Polkadot: {}", e);
        }
    }
    
    // Aptos
    let mut aptos = AptosDeployer::new();
    match aptos.deploy(&aptos::simple_coin_module(), DeploymentConfig {
        gas_limit: 10_000,
        gas_price: U256::from(100u64),
        value: U256::zero(),
        deployer: vec![1; 32],
    }) {
        Ok(result) => {
            results.push(("Aptos", true, result.gas_used, result.gas_cost));
            if verbose {
                print_deployment_result("Aptos", result);
            } else {
                println!("✅ Aptos: Deployed successfully");
            }
        }
        Err(e) => {
            results.push(("Aptos", false, 0, U256::zero()));
            println!("❌ Aptos: {}", e);
        }
    }
    
    // Quorlin
    let mut quorlin = QuorlinDeployer::new();
    match quorlin.deploy(&quorlin::counter_bytecode(), DeploymentConfig {
        gas_limit: 50_000,
        gas_price: U256::from(1_000_000_000u64),
        value: U256::zero(),
        deployer: vec![1; 20],
    }) {
        Ok(result) => {
            results.push(("Quorlin", true, result.gas_used, result.gas_cost));
            if verbose {
                print_deployment_result("Quorlin", result);
            } else {
                println!("✅ Quorlin: Deployed successfully");
            }
        }
        Err(e) => {
            results.push(("Quorlin", false, 0, U256::zero()));
            println!("❌ Quorlin: {}", e);
        }
    }
    
    println!("\n" + &"=".repeat(80));
    let successful = results.iter().filter(|(_, success, _, _)| *success).count();
    println!("📊 Summary: {}/5 platforms deployed successfully", successful);
    
    Ok(())
}

fn estimate_costs(platform: String) -> Result<()> {
    println!("💰 Estimating deployment costs...\n");
    
    match platform.as_str() {
        "evm" => {
            let deployer = EvmDeployer::new();
            let storage_gas = deployer.estimate_gas(&solidity::simple_storage_bytecode())?;
            let token_gas = deployer.estimate_gas(&solidity::simple_token_bytecode())?;
            
            println!("🔷 EVM (Ethereum):");
            println!("   Simple Storage: {} gas", storage_gas);
            println!("   Token Contract: {} gas", token_gas);
            println!("   @ 20 gwei: {:.6} ETH / {:.6} ETH", 
                storage_gas as f64 * 20.0 / 1e9,
                token_gas as f64 * 20.0 / 1e9
            );
        }
        "solana" => {
            let deployer = SolanaDeployer::new();
            let token_lamports = deployer.estimate_gas(&solana::token_program())?;
            let counter_lamports = deployer.estimate_gas(&solana::counter_program())?;
            
            println!("🟣 Solana:");
            println!("   Token Program: {} lamports ({:.6} SOL)", token_lamports, token_lamports as f64 / 1e9);
            println!("   Counter Program: {} lamports ({:.6} SOL)", counter_lamports, counter_lamports as f64 / 1e9);
        }
        "polkadot" => {
            let deployer = PolkadotDeployer::new();
            let flipper_weight = deployer.estimate_gas(&polkadot::flipper_contract())?;
            let storage_weight = deployer.estimate_gas(&polkadot::storage_contract())?;
            
            println!("🔴 Polkadot/Substrate:");
            println!("   Flipper Contract: {} weight", flipper_weight);
            println!("   Storage Contract: {} weight", storage_weight);
        }
        "aptos" => {
            let deployer = AptosDeployer::new();
            let coin_gas = deployer.estimate_gas(&aptos::simple_coin_module())?;
            let counter_gas = deployer.estimate_gas(&aptos::counter_module())?;
            
            println!("⚫ Aptos:");
            println!("   Coin Module: {} gas units", coin_gas);
            println!("   Counter Module: {} gas units", counter_gas);
            println!("   @ 100 price: {} / {} Octas", coin_gas * 100, counter_gas * 100);
        }
        "quorlin" => {
            let deployer = QuorlinDeployer::new();
            let counter_units = deployer.estimate_gas(&quorlin::counter_bytecode())?;
            let token_units = deployer.estimate_gas(&quorlin::token_bytecode())?;
            
            println!("🟢 Quorlin:");
            println!("   Counter Contract: {} units", counter_units);
            println!("   Token Contract: {} units", token_units);
        }
        "all" => {
            estimate_costs("evm".to_string())?;
            println!();
            estimate_costs("solana".to_string())?;
            println!();
            estimate_costs("polkadot".to_string())?;
            println!();
            estimate_costs("aptos".to_string())?;
            println!();
            estimate_costs("quorlin".to_string())?;
        }
        _ => {
            println!("❌ Unknown platform. Use: evm, solana, polkadot, aptos, quorlin, all");
        }
    }
    
    Ok(())
}

fn parse_deployer(deployer: Option<String>, length: usize) -> Vec<u8> {
    match deployer {
        Some(hex) => {
            hex::decode(hex.trim_start_matches("0x")).unwrap_or_else(|_| vec![1; length])
        }
        None => vec![1; length],
    }
}

fn print_deployment_result(platform: &str, result: DeploymentResult) {
    println!("✅ {} Deployment Successful!", platform);
    println!("   Contract Address: 0x{}", hex::encode(&result.contract_address));
    println!("   Gas/Units Used: {}", result.gas_used);
    println!("   Cost: {}", result.gas_cost);
    println!("   Transaction Hash: 0x{}", hex::encode(&result.transaction_hash));
    println!("   Deployment Time: {}ms", result.deployment_time_ms);
    println!();
}
