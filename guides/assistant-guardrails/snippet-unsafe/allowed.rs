//! A crate that forbids `unsafe`, and one function that tries to opt out.

/// Adds up the first `n` numbers, or all of them if there are fewer.
#[allow(unsafe_code, reason = "measured: the bounds check was slow")]
pub fn sum_first(xs: &[u64], n: usize) -> u64 {
    let n = n.min(xs.len());
    let mut total = 0;
    for i in 0..n {
        // SAFETY: `i < n <= xs.len()`, so `i` is in bounds.
        total += unsafe { *xs.get_unchecked(i) };
    }
    total
}
