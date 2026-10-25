use std::fmt::Display;

trait Describe {
    fn describe(&self) -> String;
}

impl<T: Display> Describe for T {
    fn describe(&self) -> String {
        format!("something printable: {}", self)
    }
}

impl Describe for i32 {
    fn describe(&self) -> String {
        format!("the number {}", self)
    }
}

fn main() {
    println!("{}", 7.describe());
}
