use bench_demo::parse_seconds;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn parse(c: &mut Criterion) {
    // black_box hides the input from the optimiser, so it cannot precompute the answer.
    c.bench_function("parse 1h30m15s", |b| {
        b.iter(|| parse_seconds(black_box("1h30m15s")))
    });
}

criterion_group!(benches, parse);
criterion_main!(benches);
