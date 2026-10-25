/// "Ada Lovelace" -> "AL". Words are separated by any whitespace.
fn initials(name: &str) -> String {
    // This uses iterator methods from the Iterators lesson; a for loop works too
    // (see initials_with_a_loop below).
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .flat_map(char::to_uppercase)
        .collect()
}

/// The same thing with a plain for loop.
#[allow(dead_code)]
fn initials_with_a_loop(name: &str) -> String {
    let mut out = String::new();
    for word in name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            // to_uppercase can give more than one char ('ß' becomes "SS").
            for upper in first.to_uppercase() {
                out.push(upper);
            }
        }
    }
    out
}

fn main() {
    assert_eq!(initials("Ada Lovelace"), "AL");
    assert_eq!(initials("  grace   brewster hopper "), "GBH");
    assert_eq!(initials("Émilie du Châtelet"), "ÉDC");
    assert_eq!(initials(""), "");
    println!("all checks passed");
}
