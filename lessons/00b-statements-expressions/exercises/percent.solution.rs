/// What percent `part` is of `whole`, rounded down. When `whole` is 0 the answer is 0.
fn percent(part: u32, whole: u32) -> u32 {
    if whole == 0 {
        return 0;
    }
    part * 100 / whole
}

fn main() {
    assert_eq!(
        percent(1, 4),
        25,
        "1 of 4 is 25 percent; `/` already rounds down"
    );
    assert_eq!(percent(1, 3), 33); // rounded down
    assert_eq!(percent(150, 100), 150);
    assert_eq!(percent(0, 9), 0);
    assert_eq!(percent(3, 0), 0); // must not divide by zero
    assert_eq!(percent(0, 0), 0);
    println!("all checks passed");
}
