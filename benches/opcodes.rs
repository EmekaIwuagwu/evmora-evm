use criterion::{black_box, criterion_group, criterion_main, Criterion};
use evmora_core::evm::Stack;
use primitive_types::U256;

fn bench_stack_push(c: &mut Criterion) {
    c.bench_function("stack_push", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for _ in 0..100 {
                stack.push(black_box(U256::one())).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_stack_push);
criterion_main!(benches);
