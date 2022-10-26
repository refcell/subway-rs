use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subway_rs::uniswap::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("uniswap v2 router", |b| {
        b.iter(|| get_univ2_router_address())
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
