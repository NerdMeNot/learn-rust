// Integration tests use the crate from the outside, like any other user would.
use temperature::{parse_celsius, to_fahrenheit};

#[test]
fn round_trip() {
    let celsius = parse_celsius("68F").unwrap();
    assert_eq!(to_fahrenheit(celsius), 68.0);
}
