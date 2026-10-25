/// The sum of the decimal digits of `n`, ignoring the sign.
fn digit_sum(n: i64) -> u32 {
    let mut rest = n.unsigned_abs();
    let mut sum = 0;
    while rest > 0 {
        // A digit is below 10, so this `as` cannot cut anything.
        sum += (rest % 10) as u32;
        rest /= 10;
    }
    sum
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
