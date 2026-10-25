// Backs sentences about derived order and total_cmp. Not shown on the page.
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ByNumber {
    First = 2,
    Second = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ByPlace {
    First,
    Second,
}

#[derive(Debug, PartialEq, Clone)]
struct Version {
    major: u32,
    minor: u32,
}

fn main() {
    // a derived order on an enum follows the discriminants, which are the declaration order
    // unless you write the numbers
    assert!(ByPlace::First < ByPlace::Second);
    assert!(ByNumber::First > ByNumber::Second);
    // total_cmp: the order Java's Double.compare gives (NaN last, -0.0 before 0.0)
    assert_eq!(f64::NAN.total_cmp(&1.0), Ordering::Greater);
    assert_eq!(0.0_f64.total_cmp(&-0.0), Ordering::Greater);
    // f64 has inherent max and min, which is why they work on floats without Ord
    assert_eq!(1.0_f64.max(2.0), 2.0);
    // struct update syntax: the Rust version of Kotlin's copy(minor = 5)
    let v = Version { major: 1, minor: 2 };
    assert_eq!(
        Version {
            minor: 5,
            ..v.clone()
        },
        Version { major: 1, minor: 5 }
    );
    println!("all checks passed");
}
