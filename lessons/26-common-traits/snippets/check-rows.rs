// Backs the "To do X, implement Y" table: one use of each row. Not shown on the page.
use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::ops::{Add, Deref};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Cents(u32);

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, self.0 % 100)
    }
}

impl From<u32> for Cents {
    fn from(n: u32) -> Cents {
        Cents(n)
    }
}

impl TryFrom<i64> for Cents {
    type Error = String;
    fn try_from(n: i64) -> Result<Cents, String> {
        u32::try_from(n).map(Cents).map_err(|e| e.to_string())
    }
}

#[derive(Debug)]
struct BadCents;

impl fmt::Display for BadCents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "not a number of cents")
    }
}

impl Error for BadCents {}

impl FromStr for Cents {
    type Err = BadCents;
    fn from_str(s: &str) -> Result<Cents, BadCents> {
        s.parse().map(Cents).map_err(|_| BadCents)
    }
}

impl Add for Cents {
    type Output = Cents;
    fn add(self, o: Cents) -> Cents {
        Cents(self.0 + o.0)
    }
}

impl AsRef<u32> for Cents {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

fn read_cents(s: &str) -> Result<Cents, Box<dyn Error>> {
    let c: Cents = s.parse()?; // ? turns BadCents into Box<dyn Error>
    Ok(c)
}

fn double(x: impl AsRef<u32>) -> u32 {
    *x.as_ref() * 2
}

struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

struct Wrapper(String);

impl Deref for Wrapper {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

static DROPS: AtomicUsize = AtomicUsize::new(0);

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let a = Cents(150);
    assert_eq!(format!("{a:?}"), "Cents(150)"); // Debug
    assert_eq!(format!("{a}"), "1.50"); // Display
    assert_eq!(a.to_string(), "1.50"); // ... and to_string comes with it
    let b = a.clone(); // Clone
    let c = a; // Copy: a is still usable
    assert!(a == b && b == c && a != Cents(1)); // PartialEq
    assert!(Cents(1) < Cents(2)); // PartialOrd
    let mut v = vec![Cents(3), Cents(1), Cents(2)];
    v.sort(); // Ord
    assert_eq!(v, [Cents(1), Cents(2), Cents(3)]);
    assert_eq!(Cents(1).max(Cents(2)), Cents(2)); // max, min and clamp are Ord methods
    assert_eq!(v.iter().copied().min(), Some(Cents(1)));
    assert_eq!(Cents(9).clamp(Cents(1), Cents(5)), Cents(5));
    let set: HashSet<Cents> = v.iter().copied().collect(); // Eq + Hash
    assert!(set.contains(&Cents(2)));
    let sorted: BTreeSet<Cents> = v.iter().copied().collect(); // Ord
    assert_eq!(sorted.len(), 3);
    let mut m = HashMap::new();
    m.insert(Cents(1), "one");
    assert_eq!(m[&Cents(1)], "one");
    assert_eq!(Cents::default(), Cents(0)); // Default
    let from: Cents = 5.into(); // From gives Into
    assert_eq!(from, Cents(5));
    assert_eq!(Cents::try_from(7_i64), Ok(Cents(7))); // TryFrom
    assert!(Cents::try_from(-7_i64).is_err());
    let t: Result<Cents, _> = 8_i64.try_into(); // TryInto
    assert_eq!(t, Ok(Cents(8)));
    assert_eq!("42".parse::<Cents>().unwrap(), Cents(42)); // FromStr
    assert!("x".parse::<Cents>().is_err());
    assert_eq!(double(Cents(21)), 42); // AsRef
    assert_eq!(Cents(1) + Cents(2), Cents(3)); // Add
    assert_eq!(Countdown(3).collect::<Vec<_>>(), [3, 2, 1]); // Iterator
    let mut total = 0;
    for n in Countdown(3) {
        total += n; // a for loop uses IntoIterator, which every Iterator has
    }
    assert_eq!(total, 6);
    assert!(read_cents("12").is_ok()); // Error
    assert_eq!(
        read_cents("x").unwrap_err().to_string(),
        "not a number of cents"
    );
    let w = Wrapper(String::from("hello"));
    assert_eq!(w.len(), 5); // Deref: str methods on Wrapper
    assert!(w.starts_with("he"));
    {
        let _g = Guard;
        assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    } // Drop
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    {
        // a move runs no code and drops nothing: one value, one drop
        let g = Guard;
        let h = g;
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
        drop(h);
    }
    assert_eq!(DROPS.load(Ordering::SeqCst), 2);
    println!("all checks passed");
}
