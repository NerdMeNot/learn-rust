use std::fmt::Display;

struct Wrapper<T>(T);

impl<T: Display> Wrapper<T> {
    fn show(&self) -> String {
        format!("[{}]", self.0)
    }
}

fn main() {
    println!("{}", Wrapper(vec![1, 2]).show());
}
