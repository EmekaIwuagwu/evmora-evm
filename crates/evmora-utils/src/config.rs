use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EvmConfig {
    pub chain_id: u64,
    pub version: String,
    pub gas: GasConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GasConfig {
    pub base_cost_multiplier: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub max_call_depth: usize,
    pub overflow_checks: bool,
}

impl Default for EvmConfig {
    fn default() -> Self {
        Self {
            chain_id: 1,
            version: "shanghai".to_string(),
            gas: GasConfig {
                base_cost_multiplier: 1.0,
            },
            security: SecurityConfig {
                max_call_depth: 1024,
                overflow_checks: true,
            },
        }
    }
}
