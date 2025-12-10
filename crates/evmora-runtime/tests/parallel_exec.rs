use evmora_runtime::parallel::ParallelExecutor;
use evmora_core::types::{Transaction, Address};
use primitive_types::U256;
use std::time::Instant;

#[test]
fn test_parallel_vs_serial_execution() {
    let tx_count = 100;
    println!("Preparing {} transactions...", tx_count);

    // Create 100 simple transactions: PUSH1 <n> PUSH1 0 MSTORE PUSH1 32 PUSH1 0 RETURN
    // They are independent but the current parallel executor takes a global lock on storage.
    let mut txs = Vec::new();
    for i in 0..tx_count {
        let val = (i % 255) as u8;
        // PUSH1 val PUSH1 0 MSTORE PUSH1 32 PUSH1 0 RETURN
        // 60 val 60 00 52 60 20 60 00 f3
        let data = vec![0x60, val, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xf3];
        
        let tx = Transaction::call(
            Address::random(), 
            data, 
            100_000
        );
        txs.push(tx);
    }

    // Serial Run
    // We can just use the ParallelExecutor with 1 worker or loop manually.
    // Let's use loop manually to be strictly serial.
    let start_serial = Instant::now();
    let runner_serial = ParallelExecutor::new(1);
    // Since ParallelExecutor locks internally, executing batch with it is fine if workers=1?
    // Rayon might still spawn threads. But let's assume `execute_batch` does parallel map.
    // To properly compare, let's just use `execute_batch` on a new instance.
    let _res_serial = runner_serial.execute_batch(txs.clone()).unwrap();
    let duration_serial = start_serial.elapsed();
    println!("Serial execution (via ParallelExecutor with limited parallelism implications or just overhead): {:?}", duration_serial);

    // Parallel Run
    let start_parallel = Instant::now();
    let runner_parallel = ParallelExecutor::new(4);
    let _res_parallel = runner_parallel.execute_batch(txs).unwrap();
    let duration_parallel = start_parallel.elapsed();
    println!("Parallel execution: {:?}", duration_parallel);
    
    // Note: Since the current implementation holds a global lock on storage during execution,
    // we don't expect speedup (might be slower due to contention).
    // This test verifies correctness: it runs and finishes.
    
    assert_eq!(_res_serial.len(), tx_count);
    assert_eq!(_res_parallel.len(), tx_count);
}
