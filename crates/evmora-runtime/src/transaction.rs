use evmora_core::types::Transaction;

pub struct TransactionPool {
    pub pending: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }
    
    pub fn add(&mut self, tx: Transaction) {
        self.pending.push(tx);
    }
}
