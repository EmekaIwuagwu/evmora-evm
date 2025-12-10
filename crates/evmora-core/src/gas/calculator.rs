use evmora_plugins::GasCalculator;
use evmora_plugins::EvmPlugin;
use anyhow::Result;

pub struct StandardGasCalculator;

impl EvmPlugin for StandardGasCalculator {
    fn name(&self) -> &str {
        "StandardGasCalculator"
    }
    
    fn version(&self) -> &str {
        "1.0"
    }
    
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl GasCalculator for StandardGasCalculator {
    fn calculate_gas(&self, opcode: u8, _stack_depth: usize) -> u64 {
        match opcode {
            0x00 => 0, // STOP
            0x01..=0x0b => 3, // Arithmetic
            0x20 => 30, // KECCAK256 (base)
            0x54 => 100, // SLOAD (warm, simplified)
            0x55 => 100, // SSTORE
            _ => 1, // Default (very simplified)
        }
    }
}
