use std::collections::HashMap;
use crate::traits::EvmPlugin;
use anyhow::Result;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn EvmPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    pub fn register_plugin(&mut self, plugin: Box<dyn EvmPlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }
    
    pub fn initialize_all(&mut self) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.initialize()?;
        }
        Ok(())
    }
}
