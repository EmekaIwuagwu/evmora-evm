use evmora_core::evm::InMemoryStorage;

pub struct StateManager {
    pub storage: InMemoryStorage,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
             storage: InMemoryStorage::new(),
        }
    }
}
