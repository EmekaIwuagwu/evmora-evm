use criterion::{black_box, criterion_group, criterion_main, Criterion};
use evmora_runtime::{EvmClient, Transaction};
use evmora_core::types::Address;
use tokio::runtime::Runtime;

fn bench_basic_execution(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut client = EvmClient::new("bench.toml").unwrap();
    
    // PUSH1 42 PUSH1 0 MSTORE PUSH1 32 PUSH1 0 RETURN
    let code = hex::decode("602a60005260206000f3").unwrap();
    let tx = Transaction::create(
        code,
        vec![],
        1_000_000,
    );

    c.bench_function("execute_create_return", |b| {
        b.to_async(&rt).iter(|| async {
            let tx_clone = tx.clone();
            // Client is mutable, we need to clone it or create new one?
            // Client execution modifies storage.
            // For simple code above, it only writes to memory and returns data, 
            // but it bumps nonce and creates account.
            // Using same client will accum code?
            // Actually `create` transaction makes a NEW contract.
            // So we just re-execute deployment.
            // But we need &mut client.
            // Criterion async iter is tricky with mutable self.
            // We'll create client inside check? heavy setup.
            // Or use RefCell?
            // Let's create new client inside for full E2E measurement including setup overhead 
            // OR use a client that resets.
            
            // For now, accept setup overhead to ensure isolation
            let mut local_client = EvmClient::new("bench.toml").unwrap();
            black_box(local_client.execute(black_box(tx_clone)).await.unwrap());
        })
    });
}

criterion_group!(benches, bench_basic_execution);
criterion_main!(benches);
