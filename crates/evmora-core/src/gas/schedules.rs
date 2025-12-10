#[derive(Debug, Clone)]
pub struct GasSchedule {
    pub step: u64,
    pub sload: u64,
    pub sstore_set: u64,
    pub sstore_reset: u64,
    pub memory: u64,
}

impl GasSchedule {
    pub fn london() -> Self {
        Self {
            step: 1,
            sload: 2100,
            sstore_set: 20000,
            sstore_reset: 2900,
            memory: 3,
        }
    }
}
