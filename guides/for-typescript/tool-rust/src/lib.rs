/// Turns a title into a URL slug: lowercase words joined by `-`.
///
/// ```
/// assert_eq!(slug::slugify("Hello, World"), "hello-world");
/// ```
pub fn slugify(title: &str) -> String {
    title
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_words() {
        assert_eq!(slugify("Rust for TypeScript developers"), "rust-for-typescript-developers");
    }

    #[test]
    fn drops_punctuation() {
        assert_eq!(slugify("  Hello,   World!  "), "hello-world");
    }
}
