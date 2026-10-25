//! A property test with no crate: a seeded random generator, a loop, and shrinking by hand.

use std::panic::{self, catch_unwind};
use timekeep::{average, average_naive};

/// A tiny linear congruential generator. The same seed gives the same numbers on every machine.
struct Lcg(u64);

impl Lcg {
    fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }

    /// 1 to 6 samples, each any `i32`.
    fn samples(&mut self) -> Vec<i32> {
        let len = 1 + self.next_u32() % 6;
        (0..len).map(|_| self.next_u32() as i32).collect()
    }
}

type Average = fn(&[i32]) -> Option<i32>;

/// The same seed gives the same inputs, so a failure can be replayed.
const SEED: u64 = 42;

/// The property: an average lies between the smallest and the largest sample.
/// A panic counts as a failure.
fn holds(f: Average, samples: &[i32]) -> bool {
    match catch_unwind(|| f(samples)) {
        Ok(Some(avg)) => {
            avg >= *samples.iter().min().unwrap() && avg <= *samples.iter().max().unwrap()
        }
        _ => false,
    }
}

/// Run `cases` random inputs; return the first that breaks the property.
fn find_failure(f: Average, seed: u64, cases: u32) -> Option<(u32, Vec<i32>)> {
    let mut rng = Lcg(seed);
    (0..cases).find_map(|n| {
        let samples = rng.samples();
        (!holds(f, &samples)).then_some((n, samples))
    })
}

/// Shrinking by hand: keep making the input smaller while it still fails.
/// Two moves: drop one sample, or halve one sample.
fn shrink(f: Average, mut samples: Vec<i32>) -> Vec<i32> {
    loop {
        let mut candidates = Vec::new();
        for i in 0..samples.len() {
            if samples.len() > 1 {
                let mut shorter = samples.clone();
                shorter.remove(i);
                candidates.push(shorter);
            }
            if samples[i] != 0 {
                let mut halved = samples.clone();
                halved[i] /= 2;
                candidates.push(halved);
            }
        }
        match candidates.into_iter().find(|c| !holds(f, c)) {
            Some(smaller) => samples = smaller,
            None => return samples,
        }
    }
}

#[test]
fn naive_average_has_a_counterexample() {
    // The overflow panics are expected, so keep their messages out of the output. A panic hook
    // is global to the process: a test running at the same time would lose its message too.
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let (n, found) = find_failure(average_naive, SEED, 1000).expect("a failing input");
    let small = shrink(average_naive, found.clone());
    let still_fails = !holds(average_naive, &small);
    panic::set_hook(default_hook);
    println!("seed {SEED}, case {n}: {found:?}");
    println!("shrunk to: {small:?}");
    assert!(still_fails);
    // Minimal for these moves: dropping any sample, or halving any sample, makes it pass.
    assert!(small.len() >= 2);
}

#[test]
fn fixed_average_holds_for_ten_thousand_cases() {
    assert_eq!(find_failure(average, SEED, 10_000), None);
}
