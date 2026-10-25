// A door has two switches: is it open, is it locked?
//   closed and unlocked -> "closed"
//   closed and locked   -> "locked"
//   open and unlocked   -> "open"
//   open and locked     -> "jammed" (it is open, yet the bolt is out)
fn door(open: bool, locked: bool) -> &'static str {
    match (open, locked) {
        (false, false) => "closed",
        (false, true) => "locked",
        (true, false) => "open",
        (true, true) => "jammed",
    }
}

fn main() {
    assert_eq!(door(false, false), "closed");
    assert_eq!(door(false, true), "locked");
    assert_eq!(door(true, false), "open");
    assert_eq!(door(true, true), "jammed");
    println!("all checks passed");
}
