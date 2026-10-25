use std::time::Instant;

// How many steps the Collatz process takes to get from n to 1.
fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

fn main() {
    let limit = 300_000;
    println!("debug assertions on: {}", cfg!(debug_assertions));

    let start = Instant::now();
    let mut with_loop = 0;
    for n in 1..=limit {
        with_loop += collatz_steps(n);
    }
    let loop_time = start.elapsed();

    let start = Instant::now();
    let with_iter: u64 = (1..=limit).map(collatz_steps).sum();
    let iter_time = start.elapsed();

    // The two versions are the same computation, so the answers must match.
    assert_eq!(with_loop, with_iter);
    println!("total steps for 1..={limit}: {with_iter}");
    println!("for loop:       {loop_time:?}");
    println!("iterator chain: {iter_time:?}");
}
