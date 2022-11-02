use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ethers::prelude::*;
use subway_rs::utils::sort_tokens;

fn bench_token_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("TokenSorting");
    for i in 0..100u64 {
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
    }
    group.finish();
}

criterion_group!(utils, bench_token_sorting);
criterion_main!(utils);
