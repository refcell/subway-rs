use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ethers::prelude::*;
use subway_rs::utils::{calculate_next_block_base_fee, sort_tokens};

fn bench_token_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("TokenSorting");
    let i = 1u64;
    let addr_a = Address::random();
    let addr_b = Address::random();
    group.bench_with_input(
        BenchmarkId::new("MemSwapped", &i),
        &(addr_a, addr_b),
        |b, (aa, ab)| b.iter(|| sort_tokens(&mut aa.clone(), &mut ab.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("Standard", i),
        &(&addr_a, &addr_b),
        |b, (aa, ab)| {
            b.iter(|| {
                let mut tokens = vec![aa, ab];
                tokens.sort();
            })
        },
    );
    group.finish();
}

fn bench_calculate_next_block_base_fee(c: &mut Criterion) {
    c.bench_function("Block Base Fee Calculator", |b| {
        b.iter(|| {
            let block: Block<TxHash> = Block::default();
            calculate_next_block_base_fee(block)
        })
    });
}

criterion_group! {
    name = utils;
    config = Criterion::default();
    targets = bench_token_sorting, bench_calculate_next_block_base_fee
}

criterion_main!(utils);
