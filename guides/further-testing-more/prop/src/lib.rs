//! The same two functions as in the timekeep crate, so this crate stands on its own.

/// Adds up in `i32`, so a few large samples overflow.
pub fn average_naive(samples: &[i32]) -> Option<i32> {
    if samples.is_empty() {
        return None;
    }
    Some(samples.iter().sum::<i32>() / samples.len() as i32)
}

/// Adds up in `i64`.
pub fn average(samples: &[i32]) -> Option<i32> {
    if samples.is_empty() {
        return None;
    }
    let sum: i64 = samples.iter().map(|&s| i64::from(s)).sum();
    Some((sum / samples.len() as i64) as i32)
}
