// Backs the path rows of the table.
use std::cmp::*;
use std::collections::HashMap as Map;
use std::f64::consts::PI;

mod outer {
    pub fn who() -> &'static str {
        "outer"
    }

    pub mod inner {
        pub fn up() -> &'static str {
            super::who()
        }

        pub fn here() -> &'static str {
            self::deep()
        }

        fn deep() -> &'static str {
            "inner"
        }

        pub fn top() -> &'static str {
            crate::outer::who()
        }
    }

    pub struct Pair {
        pub a: i32,
        b: i32,
    }

    impl Pair {
        pub fn new(a: i32) -> Pair {
            Pair { a, b: a * 2 }
        }
        pub fn b(&self) -> i32 {
            self.b
        }
    }
}

// no header or forward declaration: `later` is used above where it is written
fn main() {
    assert_eq!(later(), 1);
    assert_eq!(max(1, 2), 2);
    let mut m: Map<&str, i32> = Map::new();
    m.insert("a", 1);
    assert_eq!(m.len(), 1);
    assert!(PI > 3.14);
    assert_eq!(outer::inner::up(), "outer");
    assert_eq!(outer::inner::here(), "inner");
    assert_eq!(outer::inner::top(), "outer");
    let p = outer::Pair::new(4);
    // a pub field reads with `.`; the private one only through a method
    assert_eq!((p.a, p.b()), (4, 8));
    // `::` on a type, `.` on a value
    assert_eq!(String::len(&String::from("ab")), String::from("ab").len());
    println!("paths check out");
}

fn later() -> i32 {
    1
}
