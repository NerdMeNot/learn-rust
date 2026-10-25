use std::error::Error;

/// Add up numbers written like "1, 2, 3". Return Err for a piece that is not a
/// number (including an empty piece, as in "1,,2") and for a sum that does not fit
/// in an i32. Text with only spaces has no numbers: its sum is 0.
fn sum_numbers(text: &str) -> Result<i32, Box<dyn Error>> {
    todo!("split on commas, parse each piece with ?, add with checked_add")
}

fn main() {
    assert_eq!(sum_numbers("1, 2, 3").unwrap(), 6);
    assert_eq!(sum_numbers("-5, 5").unwrap(), 0);
    assert_eq!(sum_numbers("").unwrap(), 0);
    assert_eq!(sum_numbers("   ").unwrap(), 0);
    assert!(sum_numbers("1,,2").is_err());
    assert!(sum_numbers("1, 2,").is_err());
    let message = sum_numbers("1, x").unwrap_err().to_string();
    assert!(message.contains("invalid digit"), "{message}");
    let message = sum_numbers("2147483647, 1").unwrap_err().to_string();
    assert!(message.contains("does not fit"), "{message}");
    println!("all checks passed");
}
