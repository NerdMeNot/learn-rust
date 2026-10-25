/// `part * 100 / total`, rounded down. `None` when `total` is 0 or the answer
/// does not fit in a `u32`.
fn percent(part: u32, total: u32) -> Option<u32> {
    if total == 0 {
        return None;
    }
    let scaled = u64::from(part) * 100 / u64::from(total);
    u32::try_from(scaled).ok()
}

fn main() {
    assert_eq!(percent(1, 4), Some(25));
    assert_eq!(percent(1, 3), Some(33));
    assert_eq!(percent(0, 7), Some(0));
    assert_eq!(percent(150, 100), Some(150));
    // 50_000_000 * 100 does not fit in a u32.
    assert_eq!(percent(50_000_000, 100_000_000), Some(50));
    assert_eq!(percent(u32::MAX, u32::MAX), Some(100));
    // Division by zero is None, not a panic.
    assert_eq!(percent(5, 0), None);
    // The answer itself is too big for a u32.
    assert_eq!(percent(u32::MAX, 1), None);

    println!("all checks passed");
}
