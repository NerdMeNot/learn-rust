/// Turns a title into a URL slug: lowercase words joined by `-`.
///
/// ```
/// assert_eq!(slugtools::slug("Hello Rust World"), "hello-rust-world");
/// ```
pub fn slug(title: &str) -> String {
    title
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_words() {
        assert_eq!(slug("Hello Rust World"), "hello-rust-world");
    }

    #[test]
    fn ignores_extra_spaces() {
        assert_eq!(slug("  Hello   Rust "), "hello-rust");
    }
}
