use std::ops::Mul;

// Backs the C++ table: a const fn evaluated while compiling, and a generic square with a bound.
const fn square_const(x: i32) -> i32 {
    x * x
}

const SEVEN_SQUARED: i32 = square_const(7);
const _: () = assert!(SEVEN_SQUARED == 49);

fn square<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

fn main() {
    assert_eq!(SEVEN_SQUARED, 49);
    assert_eq!(square(7), 49);
    assert_eq!(square(2.5), 6.25);
    println!("const fn and generic square checked");
}
