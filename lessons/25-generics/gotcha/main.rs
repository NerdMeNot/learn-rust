use std::ops::Add;

fn add<T: Add>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
    println!("{}", add(1.5, 2.25));
}
