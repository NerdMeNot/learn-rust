use std::ops::Add;

// The sum of two Ts must also be a T: `Output = T` says so.
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
    println!("{}", add(1.5, 2.25));
}
