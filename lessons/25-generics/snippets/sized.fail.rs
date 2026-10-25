use std::fmt::Display;

fn show<T: Display>(item: &T) -> String {
    format!("<{item}>")
}

fn main() {
    println!("{}", show("hello"));
}
