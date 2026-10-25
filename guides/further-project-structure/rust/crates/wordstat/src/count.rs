use std::collections::BTreeMap;

use crate::Error;

/// How often each word appeared, with the words in alphabetical order.
#[derive(Debug, Default)]
pub struct Stats {
    counts: BTreeMap<String, usize>,
}

/// Counts the words in `text`, ignoring case and surrounding punctuation.
///
/// # Errors
///
/// Returns [`Error::Empty`] when `text` has no words.
pub fn count_words(text: &str) -> Result<Stats, Error> {
    let mut stats = Stats::default();
    for word in text.split_whitespace() {
        *stats.counts.entry(normalize(word)).or_insert(0) += 1;
    }
    if stats.counts.is_empty() {
        return Err(Error::Empty);
    }
    Ok(stats)
}

// Only this crate needs to know how words are normalised.
pub(crate) fn normalize(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

impl Stats {
    /// The number of words counted, repeats included.
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }

    /// How often `word` appeared (0 if it did not).
    pub fn get(&self, word: &str) -> usize {
        self.counts.get(word).copied().unwrap_or(0)
    }

    /// The words and their counts, most frequent first; ties in alphabetical
    /// order.
    pub fn most_common(&self) -> Vec<(&str, usize)> {
        let mut all: Vec<_> =
            self.counts.iter().map(|(w, n)| (w.as_str(), *n)).collect();
        all.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        all
    }

    // Visible to the rest of this crate (the `json` module), not to users.
    #[cfg_attr(not(feature = "json"), allow(dead_code))]
    pub(crate) fn counts(&self) -> &BTreeMap<String, usize> {
        &self.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_case_and_punctuation() {
        let stats = count_words("Go, go! GO?").unwrap();
        assert_eq!(stats.get("go"), 3);
    }

    #[test]
    fn empty_text_is_an_error() {
        assert_eq!(count_words("  ").unwrap_err(), Error::Empty);
    }
}
