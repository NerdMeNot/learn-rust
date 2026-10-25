// Python's `//` rounds toward minus infinity: -7 // 2 is -4.
// Rust's `/` on integers cuts toward zero: -7 / 2 is -3.
// Write floor_div for a divisor that is not zero.
fn floor_div(a: i32, b: i32) -> i32 {
    let quotient = a / b;
    if a % b != 0 && ((a < 0) != (b < 0)) {
        quotient - 1
    } else {
        quotient
    }
}

fn main() {
    assert_eq!(floor_div(7, 2), 3);
    assert_eq!(floor_div(-7, 2), -4);
    assert_eq!(floor_div(7, -2), -4);
    assert_eq!(floor_div(-7, -2), 3);
    assert_eq!(floor_div(6, 3), 2);
    assert_eq!(floor_div(-6, 3), -2);
    assert_eq!(floor_div(0, 5), 0);
    println!("all checks passed");
}
