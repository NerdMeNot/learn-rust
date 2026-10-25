/// The length of the longest stretch where every reading is higher than the one
/// before it. A single reading on its own is a stretch of 1.
fn longest_rise(readings: &[i32]) -> usize {
    readings
        .chunk_by(|a, b| a < b)
        .map(|run| run.len())
        .max()
        .unwrap_or(0)
}

fn main() {
    assert_eq!(longest_rise(&[1, 2, 3, 1, 2]), 3);
    assert_eq!(longest_rise(&[3, 1, 2, 3, 4, 0]), 4);
    assert_eq!(longest_rise(&[5, 4, 3]), 1);
    // Equal is not rising.
    assert_eq!(longest_rise(&[2, 2, 2]), 1);
    assert_eq!(longest_rise(&[7]), 1);
    // No readings, no run.
    assert_eq!(longest_rise(&[]), 0);
    println!("all checks passed");
}
