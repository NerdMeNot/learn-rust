#![feature(test)]
extern crate test;

pub fn double(n: u64) -> u64 {
    n * 2
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_double(b: &mut Bencher) {
        b.iter(|| double(test::black_box(21)));
    }
}
