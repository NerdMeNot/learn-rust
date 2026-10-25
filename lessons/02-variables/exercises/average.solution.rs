/// The average of `total` spread over `count` items, as a float.
/// With no items the average is 0.0.
fn average(total: i32, count: i32) -> f64 {
    if count == 0 {
        return 0.0;
    }
    let total = total as f64;
    let count = count as f64;
    total / count
}

fn main() {
    assert_eq!(average(7, 2), 3.5);
    assert_eq!(average(10, 5), 2.0);
    assert_eq!(average(-3, 2), -1.5);
    // The case to watch: no items must give 0.0, not NaN.
    assert_eq!(average(0, 0), 0.0);
    assert_eq!(average(5, 0), 0.0);
    println!("all checks passed");
}
