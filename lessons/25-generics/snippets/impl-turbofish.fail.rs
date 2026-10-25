use std::fmt::Display;

fn named<T: Display>(x: T) -> String {
    format!("{x}")
}

fn hidden(x: impl Display) -> String {
    format!("{x}")
}

fn main() {
    println!("{}", named::<i32>(1));
    println!("{}", hidden::<i32>(1));
}
