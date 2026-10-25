/// The sum of the decimal digits of `n`, ignoring the sign.
fn digit_sum(n: i64) -> u32 {
    todo!("take the absolute value as a u64 (unsigned_abs), then use % 10 and / 10 in a loop; a digit is below 10, so `as u32` cannot cut anything")
}

fn main() {
    assert_eq!(digit_sum(0), 0);
    assert_eq!(digit_sum(7), 7);
    assert_eq!(digit_sum(1234), 10);
    assert_eq!(digit_sum(-1234), 10);
    assert_eq!(digit_sum(1_000_000), 1);
    // i64::MIN is -9223372036854775808; its absolute value does not fit in an i64.
    assert_eq!(digit_sum(i64::MIN), 89);
    assert_eq!(digit_sum(i64::MAX), 88);

    println!("all checks passed");
}
