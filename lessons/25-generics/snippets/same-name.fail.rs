use std::fmt::{Debug, Display};

struct Wrapper<T>(T);

impl<T: Display> Wrapper<T> {
    fn show(&self) -> String {
        format!("{}", self.0)
    }
}

impl<T: Debug> Wrapper<T> {
    fn show(&self) -> String {
        format!("{:?}", self.0)
    }
}

fn main() {
    println!("{}", Wrapper(1).show());
}
