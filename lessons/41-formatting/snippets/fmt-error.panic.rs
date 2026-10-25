use std::fmt::{self, Write};

struct Broken;

impl fmt::Display for Broken {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error) // nothing went wrong with the destination: this is a bug
    }
}

fn main() {
    let mut text = String::new();
    println!("write! returned: {:?}", write!(text, "{}", Broken));
    println!("{}", Broken.to_string());
}
