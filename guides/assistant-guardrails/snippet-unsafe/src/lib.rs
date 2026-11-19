//! A crate that does need `unsafe`, so it cannot forbid it.

/// Adds up the first `n` numbers, or all of them if there are fewer.
pub fn sum_first(xs: &[u64], n: usize) -> u64 {
    let n = n.min(xs.len());
    let mut total = 0;
    for i in 0..n {
        total += unsafe { *xs.get_unchecked(i) };
    }
    total
}

/// The same, with the promise written down.
pub fn sum_first_documented(xs: &[u64], n: usize) -> u64 {
    let n = n.min(xs.len());
    let mut total = 0;
    for i in 0..n {
        // SAFETY: `i < n <= xs.len()`, so `i` is in bounds.
        total += unsafe { *xs.get_unchecked(i) };
    }
    total
}
