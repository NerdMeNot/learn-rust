// Backs the claim that traits give one name several meanings, while two inherent methods cannot.
struct Y(String);

impl From<u8> for Y {
    fn from(n: u8) -> Y {
        Y(format!("number {n}"))
    }
}

impl From<&str> for Y {
    fn from(s: &str) -> Y {
        Y(format!("text {s}"))
    }
}

fn main() {
    assert_eq!(Y::from(7u8).0, "number 7");
    assert_eq!(Y::from("abc").0, "text abc");
    println!("checks passed");
}
