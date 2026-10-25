use std::fmt::Display;

fn show<T: Display>(x: T) -> String {
    format!("{}: {}", std::any::type_name::<T>(), x)
}

fn main() {
    println!("{}", show(5));
    println!("{}", show("hi"));
}
