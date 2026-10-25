#![deny(nonstandard_style)]

const MAX_USERS: u32 = 3;

struct UserAccount {
    user_count: u32,
}

enum Shape {
    Circle(f64),
    Square(f64),
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("circle {r}"),
            Shape::Square(s) => format!("square {s}"),
        }
    }
}

mod parse_line {
    pub fn first_word<'a, T: AsRef<str> + ?Sized>(s: &'a T) -> &'a str {
        s.as_ref().split(' ').next().unwrap_or("")
    }
}

fn main() {
    let account = UserAccount { user_count: MAX_USERS };
    println!("{} {}", account.user_count, parse_line::first_word("hi there"));
    println!("{} {}", Shape::Circle(1.5).describe(), Shape::Square(2.0).describe());
    // method-name prefixes from the standard library
    let s = String::from("Hi");
    let v = vec![1, 2];
    println!("{} {} {}", s.as_str(), s.to_uppercase(), v.is_empty());
    println!("{:?}", s.into_bytes());
}
