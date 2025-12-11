// Backend-specific semantic validation

use super::types::{Type, SemanticError, SemanticResult};
use super::ast::Contract;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Backend {
    EVM,
    Solana,
    Polkadot,
    Aptos,
    Quorlin,
}

pub struct BackendValidator {
    backend: Backend,
}

impl BackendValidator {
    pub fn new(backend: Backend) -> Self {
        Self { backend }
    }

    pub fn validate_contract(&self, contract: &Contract) -> SemanticResult<()> {
        match self.backend {
            Backend::EVM => self.validate_evm(contract),
            Backend::Solana => self.validate_solana(contract),
            Backend::Polkadot => self.validate_polkadot(contract),
            Backend::Aptos => self.validate_aptos(contract),
            Backend::Quorlin => Ok(()), // No special restrictions
        }
    }

    fn validate_evm(&self, _contract: &Contract) -> SemanticResult<()> {
        // EVM-specific validations
        // - Max contract size: 24KB
        // - Function selector collisions
        // - Storage slot limits
        Ok(())
    }

    fn validate_solana(&self, contract: &Contract) -> SemanticResult<()> {
        // Solana-specific validations
        // - All state must be in accounts
        // - No unlimited storage
        for state_var in &contract.state_vars {
            if matches!(state_var.type_, Type::Mapping(_, _)) {
                // Mappings need to be implemented as separate accounts in Solana
            }
        }
        Ok(())
    }

    fn validate_polkadot(&self, contract: &Contract) -> SemanticResult<()> {
        // ink! specific validations
        // - No u256 type support (only u128 max)
        for state_var in &contract.state_vars {
            if let Type::Simple(ref s) = state_var.type_ {
                if s == "uint256" {
                    return Err(SemanticError::TypeMismatch {
                        expected: "uint128 or lower (ink! limitation)".to_string(),
                        found: "uint256".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    fn validate_aptos(&self, _contract: &Contract) -> SemanticResult<()> {
        // Move-specific validations
        // - Resource safety
        // - Borrowing rules
        // - No cyclic references
        Ok(())
    }

    pub fn check_type_support(&self, type_: &Type) -> SemanticResult<()> {
        match self.backend {
            Backend::Polkadot => {
                // ink! doesn't support u256
                if let Type::Simple(ref s) = type_ {
                    if s == "uint256" {
                        return Err(SemanticError::TypeMismatch {
                            expected: "uint128 max for Polkadot/ink!".to_string(),
                            found: s.clone(),
                        });
                    }
                }
            }
            Backend::Solana => {
                // Solana has different account model
                if let Type::Mapping(_, _) = type_ {
                    // Warn: mappings are expensive on Solana
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn estimate_storage_cost(&self, type_: &Type) -> u64 {
        match self.backend {
            Backend::EVM => {
                // EVM: 20,000 gas for SSTORE
                match type_ {
                    Type::Simple(_) => 20000,
                    Type::Mapping(_, _) => 20000, // per slot
                    Type::List(_) => 40000, // initial + length
                    _ => 20000,
                }
            }
            Backend::Solana => {
                // Solana: rent per byte
                Self::type_size_bytes(type_) * 100 // lamports per byte
            }
            Backend::Polkadot => {
                // ink!: deposit per byte
                Self::type_size_bytes(type_) * 1000
            }
            Backend::Aptos => {
                // Move: gas units
                Self::type_size_bytes(type_) * 10
            }
            Backend::Quorlin => 0,
        }
    }

    fn type_size_bytes(type_: &Type) -> u64 {
        match type_ {
            Type::Simple(s) => {
                if s.starts_with("uint") || s.starts_with("int") {
                    let bits = s.trim_start_matches("uint")
                        .trim_start_matches("int")
                        .parse::<u64>()
                        .unwrap_or(256);
                    bits / 8
                } else if s == "address" {
                    20
                } else if s == "bool" {
                    1
                } else {
                    32
                }
            }
            Type::Mapping(_, _) => 32, // slot reference
            Type::List(_) => 32, // pointer
            Type::Tuple(ts) => ts.iter().map(Self::type_size_bytes).sum(),
            Type::Optional(_) => 33, // 1 byte flag + value
            Type::Unknown => 0,
        }
    }
}
