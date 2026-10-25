//! A crate-level `//!` doc comment.
#![allow(dead_code, unused_variables)]

mod shapes {
    /// A doc comment on a public item (`///`).
    pub struct Square(pub f64);

    pub trait Area {
        fn area(&self) -> f64;
    }

    impl Area for Square {
        fn area(&self) -> f64 {
            self.0 * self.0
        }
    }
}

use shapes::{Area, Square};

fn biggest<T>(items: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn total(shapes: &[&dyn Area]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn parse(text: &str) -> Result<i32, std::num::ParseIntError> {
    let n = text.parse::<i32>()?;
    Ok(n)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut count = 0;
    count += 1;
    let big = 1_000_000;
    let pair = (1, 'x');
    let list = [1, 2, 3];
    let first = list[0];
    let has_items = !list.is_empty();
    let r = &count;
    let m = &mut count;
    *m += 1;
    let half = 7 as f64 / 2.0;
    let label = 'x';
    'outer: for i in 0..3 {
        for j in 0..=3 {
            if j == 1 {
                continue 'outer;
            }
        }
    }
    let kind = match first {
        1 | 2 => "small",
        n @ 3..=9 => "mid",
        _ => "other",
    };
    let p = Point { x: 1, y: 2 };
    let Point { x, .. } = p;
    let sq = Square(2.0);
    let sum = total(&[&sq]);
    println!("{} {} {} {:?} {} {} {}", count, first, half, parse("7"), kind, x, sum);
    println!("{} {}", has_items, longer("ab", "abc"));
    println!("{}", biggest(&[3, 9, 4]));
}
