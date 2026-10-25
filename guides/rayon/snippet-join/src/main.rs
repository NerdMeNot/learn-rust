use rayon::prelude::*;

// Split the slice in two and let `join` run both halves, possibly on two threads.
// Below a cutoff, just add: a task per element would likely cost more than the addition.
fn sum(numbers: &[u64]) -> u64 {
    if numbers.len() <= 1_000 {
        return numbers.iter().sum();
    }
    let (left, right) = numbers.split_at(numbers.len() / 2);
    let (a, b) = rayon::join(|| sum(left), || sum(right));
    a + b
}

fn main() {
    let numbers: Vec<u64> = (1..=100_000).collect();
    println!("join: {}", sum(&numbers));
    println!("par_iter: {}", numbers.par_iter().sum::<u64>());

    // `scope` lets tasks borrow local variables, like `thread::scope`.
    // Each task gets its own slot of `results`.
    let mut results = vec![0u64; 4];
    rayon::scope(|s| {
        for (i, slot) in results.iter_mut().enumerate() {
            s.spawn(move |_| *slot = (i as u64 + 1) * 100);
        }
    });
    println!("scope: {results:?}");
}
