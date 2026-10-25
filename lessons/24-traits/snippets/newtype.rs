use std::fmt;

struct Numbers(Vec<i32>); // a wrapper type that belongs to this crate

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} numbers", self.0.len())
    }
}

fn main() {
    println!("{}", Numbers(vec![1, 2, 3]));
}
