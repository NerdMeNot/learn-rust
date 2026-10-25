use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum UsernameError {
    Empty,
    TooLong(usize),
    BadChar(char),
}

// Write the messages: "username is empty", "username has 13 characters, the
// most allowed is 12", and "username contains '!'" (with the character).
impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("match on self and write! a message into f")
    }
}

impl Error for UsernameError {}

/// A username has 1 to 12 characters, and each one is alphanumeric (letters and
/// numbers of any script count) or '_'.
/// Count characters, not bytes. Check for an empty name first, then the length,
/// then the characters.
fn check_username(name: &str) -> Result<&str, UsernameError> {
    todo!("check empty, then the character count, then each character")
}

fn main() {
    assert_eq!(check_username("ada_99"), Ok("ada_99"));
    assert_eq!(check_username(""), Err(UsernameError::Empty));
    assert_eq!(check_username("a b"), Err(UsernameError::BadChar(' ')));
    // 12 characters but 24 bytes: fine. 13 characters: too long.
    assert_eq!(check_username("éééééééééééé"), Ok("éééééééééééé"));
    assert_eq!(
        check_username("ééééééééééééé"),
        Err(UsernameError::TooLong(13))
    );
    assert_eq!(check_username("josé!"), Err(UsernameError::BadChar('!')));

    assert_eq!(UsernameError::Empty.to_string(), "username is empty");
    assert_eq!(
        UsernameError::TooLong(13).to_string(),
        "username has 13 characters, the most allowed is 12"
    );
    assert_eq!(
        UsernameError::BadChar('!').to_string(),
        "username contains '!'"
    );
    println!("all checks passed");
}
