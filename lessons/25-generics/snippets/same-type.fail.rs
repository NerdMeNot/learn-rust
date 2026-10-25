use std::fmt::Display;

// One T for both parameters: the two arguments must have the same type.
fn pair<T: Display>(a: T, b: T) -> String {
    format!("{a} and {b}")
}

fn main() {
    println!("{}", pair(1, "one"));
}
