/// Shorten `text` to at most `max` characters (not bytes), adding "…" when it was cut.
/// It must never panic, whatever the text.
fn truncate_chars(text: &str, max: usize) -> String {
    todo!("find where character number `max` starts with char_indices")
}

fn main() {
    assert_eq!(truncate_chars("hello", 10), "hello");
    assert_eq!(truncate_chars("hello world", 5), "hello…");
    assert_eq!(truncate_chars("naïve café", 4), "naïv…");
    assert_eq!(truncate_chars("🦀🦀🦀", 2), "🦀🦀…");
    assert_eq!(truncate_chars("", 3), "");
    println!("all checks passed");
}
