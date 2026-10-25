// The mean of `count` readings that add up to `total`.
// Rust will not mix an i64 and a u32 in one division: convert both yourself.
// With no readings there is no mean, so return None instead of a NaN.
fn average(total: i64, count: u32) -> Option<f64> {
    if count == 0 {
        return None;
    }
    Some(total as f64 / f64::from(count))
}

fn main() {
    assert_eq!(average(7, 2), Some(3.5));
    assert_eq!(average(-7, 2), Some(-3.5));
    assert_eq!(average(0, 4), Some(0.0));
    assert_eq!(average(0, 0), None);
    assert_eq!(average(5, 0), None);
    println!("all checks passed");
}
