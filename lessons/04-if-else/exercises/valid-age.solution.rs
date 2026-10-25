// Turn text typed by a user into an age.
// Surrounding spaces are allowed. The age must be a whole number from 0 to 120.
// Anything else gives `None`. Use `let ... else` for the parse and a plain `if` for the range.
// (`Option` is covered in a later lesson: `Some(n)` is "there is a value", `None` is "there is not".)
fn valid_age(text: &str) -> Option<i32> {
    let Ok(n) = text.trim().parse::<i32>() else {
        return None;
    };
    if n < 0 || n > 120 { None } else { Some(n) }
}

fn main() {
    assert_eq!(valid_age("42"), Some(42));
    assert_eq!(valid_age("  7 "), Some(7));
    assert_eq!(valid_age("0"), Some(0));
    assert_eq!(valid_age("120"), Some(120));
    assert_eq!(valid_age("121"), None);
    assert_eq!(valid_age("-5"), None);
    assert_eq!(valid_age("abc"), None);
    assert_eq!(valid_age("4 2"), None);
    assert_eq!(valid_age(""), None);
    assert_eq!(valid_age("99999999999"), None);
    println!("all checks passed");
}
