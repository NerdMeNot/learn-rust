// Python's `//` rounds toward minus infinity: -7 // 2 is -4.
// Rust's `/` on integers cuts toward zero: -7 / 2 is -3.
// Write floor_div for a divisor that is not zero.
fn floor_div(a: i32, b: i32) -> i32 {
    todo!("a / b, minus one if inexact and the signs differ")
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
