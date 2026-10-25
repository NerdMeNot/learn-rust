/// Splits text into words: runs of letters and apostrophes, with apostrophes
/// trimmed off the ends. The words are slices of `text`, so nothing is copied.
pub fn words(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    for piece in text.split(|c: char| !(c.is_alphabetic() || c == '\'')) {
        let word = piece.trim_matches('\'');
        if !word.is_empty() {
            found.push(word);
        }
    }
    found
}
