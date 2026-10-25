use criterion::{Criterion, criterion_group, criterion_main};
use mix_bench::mix;
use std::hint::black_box;

fn bench_mix(c: &mut Criterion) {
    // The input and the result both go through black_box, so the compiler cannot
    // precompute the answer or drop the call.
    c.bench_function("mix 1000 rounds", |b| b.iter(|| black_box(mix(black_box(88172645463325252), 1000))));
}

criterion_group!(benches, bench_mix);
criterion_main!(benches);
