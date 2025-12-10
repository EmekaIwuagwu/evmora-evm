use evmora_core::types::Transaction;
use evmora_core::evm::executor::ExecutionResult;
use evmora_core::evm::storage::InMemoryStorage;
use evmora_core::evm::context::ExecutionContext;
use evmora_core::evm::executor::Executor;
use evmora_core::gas::StandardGasCalculator;
use anyhow::Result;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct ParallelExecutor {
    workers: usize,
    storage: Arc<Mutex<InMemoryStorage>>,
}

impl ParallelExecutor {
    pub fn new(workers: usize) -> Self {
        Self {
            workers,
            storage: Arc::new(Mutex::new(InMemoryStorage::new())),
        }
    }

    pub fn execute_batch(&self, transactions: Vec<Transaction>) -> Result<Vec<ExecutionResult>> {
        // Naive parallel execution: lock storage for each tx.
        // In a real high-perf EVM, we would use optimistic execution with read/write sets 
        // to detect conflicts and re-execute only conflicting txs sequentially.
        
        let results: Vec<Result<ExecutionResult>> = transactions.into_par_iter()
            .map(|tx| {
                let mut context = ExecutionContext::default();
                context.gas_limit = tx.gas_limit;
                context.value = tx.value;
                context.data = tx.data.clone();
                if let Some(to) = tx.to {
                    context.address = to;
                }

                // Lock storage for execution
                // This defeats the purpose of parallelism for stateful ops, but works for stateless/read-only
                // To do this properly, we need a refined locking strategy or STM.
                let mut storage = self.storage.lock().unwrap();
                let gas_calculator = Box::new(StandardGasCalculator);
                
                // We use the storage reference directly.
                // Note: Executor expects `&mut dyn StorageBackend`, which we can get from the mutex guard.
                let mut executor = Executor::new(context, &mut *storage, gas_calculator);
                
                let res = executor.execute(&tx.data);
                
                // If we were doing optimistic execution, we would capture the RW set here 
                // and commit later if no conflicts.
                
                Ok(res?)
            })
            .collect();

        // Unwrap results
        results.into_iter().collect()
    }
}
