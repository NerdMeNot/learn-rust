//! The crate-level doc comment: `//!` describes the file it is in.
#![allow(dead_code)]

mod shapes {
    /// A doc comment: `///` describes the item below it.
    pub struct Square {
        pub side: u32,
    }

    impl Square {
        pub fn area(&self) -> u32 {
            self.side * self.side
        }
    }
}

use shapes::Square;
use std::cmp::max as bigger;

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Square {
    fn describe(&self) -> String {
        format!("square, side {}", self.side)
    }
}

fn show_generic<T>(item: &T) -> String
where
    T: Describe,
{
    item.describe()
}

fn show_dyn(item: &dyn Describe) -> String {
    item.describe()
}

fn first_word<'a>(text: &'a str) -> &'a str {
    text.split(' ').next().unwrap_or("")
}

fn stop() -> ! {
    std::process::exit(1)
}

fn main() {
    let sq = Square { side: 4 };
    let big = sq.area() as f64 * 1.5;
    println!("area: {}, scaled: {big}", sq.area());
    println!("generic: {}", show_generic(&sq));
    println!("dyn: {}", show_dyn(&sq));
    println!("as renames in use: {}", bigger(3, 9));
    println!("first_word: {}", first_word("hello there"));
    let label = match sq.side {
        n @ 1..=9 => format!("small ({n})"),
        _ => String::from("big"),
    };
    println!("@ binds the matched value: {label}");
    let sides: [u32; 3] = [4, 5, 6];
    println!("[] makes, types and indexes an array: {}", sides[1]);
    let nothing = ();
    println!("() is a value too: {nothing:?}");
}
