use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} numbers", self.len())
    }
}

fn main() {
    println!("{}", vec![1, 2, 3]);
}
