use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ethers::prelude::*;
use subway_rs::uniswap::*;

use criterion::async_executor::FuturesExecutor;

fn bench_univ2_router_address(c: &mut Criterion) {
    c.bench_function("uniswap v2 router", |b| {
        b.iter(|| get_univ2_router_address())
    });
}

fn bench_univ2_factory_address(c: &mut Criterion) {
    c.bench_function("uniswap v2 factory", |b| {
        b.iter(|| get_univ2_factory_address())
    });
}

fn bench_calculate_uniswap_v2_pair_address(c: &mut Criterion) {
    let addr_a = Address::random();
    let addr_b = Address::random();
    c.bench_function("calculate uniswap v2 pair addresses", |b| {
        b.iter(|| calculate_uniswap_v2_pair_address(&addr_a, &addr_b))
    });
}

fn bench_get_uniswap_v2_pair_address(c: &mut Criterion) {
    let addr_a = Address::random();
    let addr_b = Address::random();
    c.bench_function("uniswap v2 factory", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| get_uniswap_v2_pair_address(&addr_a, &addr_b))
    });
}

fn bench_pair_addresses(c: &mut Criterion) {
    let mut group = c.benchmark_group("UniswapPairAddressRetrievers");
    for i in 0..100u64 {
        let addr_a = Address::random();
        let addr_b = Address::random();
        group.bench_with_input(
            BenchmarkId::new("Calculated", &i),
            &(&addr_a, &addr_b),
            |b, (aa, ab)| b.iter(|| calculate_uniswap_v2_pair_address(aa, ab)),
        );
        group.bench_with_input(
            BenchmarkId::new("Fetched", i),
            &(&addr_a, &addr_b),
            |b, (aa, ab)| {
                b.to_async(FuturesExecutor)
                    .iter(|| get_uniswap_v2_pair_address(aa, ab))
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = uniswap_benches;
    config = Criterion::default();
    targets =
        bench_univ2_router_address,
        bench_univ2_factory_address,
        bench_calculate_uniswap_v2_pair_address,
        bench_get_uniswap_v2_pair_address,
        bench_pair_addresses
}

criterion_main!(uniswap_benches);
