use std::fmt::Display;

// The bound is on the struct, so every impl has to repeat it.
struct Labeled<T: Display> {
    value: T,
}

impl<T> Labeled<T> {
    fn value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let label = Labeled { value: 7 };
    println!("{}", label.value());
}
