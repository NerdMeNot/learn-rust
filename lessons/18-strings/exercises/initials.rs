/// "Ada Lovelace" -> "AL". Words are separated by any whitespace.
fn initials(name: &str) -> String {
    todo!("take the first char of each word, uppercased")
}

fn main() {
    assert_eq!(initials("Ada Lovelace"), "AL");
    assert_eq!(initials("  grace   brewster hopper "), "GBH");
    assert_eq!(initials("Émilie du Châtelet"), "ÉDC");
    assert_eq!(initials(""), "");
    println!("all checks passed");
}
