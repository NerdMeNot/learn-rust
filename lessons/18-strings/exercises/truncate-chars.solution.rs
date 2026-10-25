/// Shorten `text` to at most `max` characters (not bytes), adding "…" when it was cut.
/// It must never panic, whatever the text.
fn truncate_chars(text: &str, max: usize) -> String {
    // char_indices finds the byte where character number `max` starts, if there is one.
    // nth is an iterator method from the Iterators lesson; a for loop works too
    // (see truncate_chars_with_a_loop below).
    match text.char_indices().nth(max) {
        Some((byte, _)) => format!("{}…", &text[..byte]),
        None => text.to_string(),
    }
}

/// The same thing with a plain for loop.
#[allow(dead_code)]
fn truncate_chars_with_a_loop(text: &str, max: usize) -> String {
    let mut count = 0;
    for (byte, _) in text.char_indices() {
        if count == max {
            // `byte` is where a character starts, so slicing here is safe.
            return format!("{}…", &text[..byte]);
        }
        count += 1;
    }
    text.to_string()
}

fn main() {
    assert_eq!(truncate_chars("hello", 10), "hello");
    assert_eq!(truncate_chars("hello world", 5), "hello…");
    assert_eq!(truncate_chars("naïve café", 4), "naïv…");
    assert_eq!(truncate_chars("🦀🦀🦀", 2), "🦀🦀…");
    assert_eq!(truncate_chars("", 3), "");
    println!("all checks passed");
}
