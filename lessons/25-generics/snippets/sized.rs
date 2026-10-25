use std::fmt::Display;
use std::mem::size_of_val;

// `?Sized` lifts the hidden "T has a known size" rule.
fn show<T: Display + ?Sized>(item: &T) -> String {
    format!("<{item}> is {} bytes", size_of_val(item))
}

fn main() {
    println!("{}", show("hello"));
    println!("{}", show(&5_i64));
}
