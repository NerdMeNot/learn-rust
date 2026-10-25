use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let numbers: Vec<u64> = (1..=100_000).collect();

    // 1. No shared state: each piece returns a value and rayon combines them.
    let by_sum: u64 = numbers.par_iter().sum();

    // 2. One atomic counter that every closure adds to.
    let atomic = AtomicU64::new(0);
    numbers.par_iter().for_each(|n| {
        atomic.fetch_add(*n, Ordering::Relaxed);
    });

    // 3. A mutex: correct, but only one closure at a time can be inside it.
    let locked = Mutex::new(0u64);
    numbers.par_iter().for_each(|n| *locked.lock().unwrap() += n);

    println!("sum {by_sum}, atomic {}, mutex {}", atomic.into_inner(), locked.into_inner().unwrap());

    // 4. Pushing into a shared Vec: all items arrive, but in no promised order.
    let pushed = Mutex::new(Vec::new());
    numbers.par_iter().for_each(|n| pushed.lock().unwrap().push(*n));
    let mut pushed = pushed.into_inner().unwrap();
    pushed.sort();
    println!("same items after sorting: {}", pushed == numbers);

    // 5. Per-piece state: `fold` gives each piece its own counts, `reduce` merges them.
    let by_remainder = numbers
        .par_iter()
        .fold(
            || [0u64; 3],
            |mut counts, n| {
                counts[(*n % 3) as usize] += 1;
                counts
            },
        )
        .reduce(
            || [0u64; 3],
            |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]],
        );
    println!("remainders 0, 1, 2: {by_remainder:?}");
}
