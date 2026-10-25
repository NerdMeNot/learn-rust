use std::hint::black_box;
use std::time::Instant;

// A loop whose total has a short formula: the compiler can skip the looping.
fn sum_squares(n: u64) -> u64 {
    (0..n).map(|i| i.wrapping_mul(i)).fold(0, u64::wrapping_add)
}

// A loop that scrambles the bits each time round: no formula, so it has to run.
fn mix(n: u64) -> u64 {
    let mut x = 88172645463325252u64;
    for _ in 0..n {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
    }
    x
}

fn time(label: &str, work: impl FnOnce() -> u64) {
    let start = Instant::now();
    let result = work();
    println!("{label:<36} result {result:>20}, took {:?}", start.elapsed());
}

fn main() {
    time("sum_squares(100_000_000)", || sum_squares(100_000_000));
    time("sum_squares(black_box(100_000_000))", || sum_squares(black_box(100_000_000)));
    time("mix(100)", || mix(100));
    time("mix(black_box(100))", || mix(black_box(100)));
    time("mix(100_000_000)", || mix(100_000_000));
    time("mix(black_box(100_000_000))", || mix(black_box(100_000_000)));
}
