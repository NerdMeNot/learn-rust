use rayon::prelude::*;
use std::time::Instant;

// The number of steps the Collatz process takes to reach 1: different work for every n.
fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

fn compare(label: &str, count: u64) {
    let start = Instant::now();
    let a: u64 = (1..=count).map(collatz_steps).sum();
    let sequential = start.elapsed();

    let start = Instant::now();
    let b: u64 = (1..=count).into_par_iter().map(collatz_steps).sum();
    let parallel = start.elapsed();

    assert_eq!(a, b);
    println!("{label}: {count} items, sequential {sequential:?}, parallel {parallel:?}");
}

fn main() {
    // Start the pool first, so its start-up is not inside the first measurement.
    let warm: u64 = (0..1_000u64).into_par_iter().sum();
    assert_eq!(warm, 499_500);
    let cpus = std::thread::available_parallelism().unwrap().get();
    println!("CPUs reported: {cpus}, rayon threads: {}", rayon::current_num_threads());
    compare("small", 10);
    compare("large", 1_000_000);
}
