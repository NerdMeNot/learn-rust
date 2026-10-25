use std::collections::HashMap;

use crate::words::words;

/// Counts how often each word appears, ignoring case.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// The `n` most common words, most common first. Words with the same count
/// come out in code point order (for English, plain alphabetical order).
pub fn top_n(counts: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    entries.truncate(n);
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_case_and_punctuation() {
        let counts = count_words("The cat. the CAT, the dog!");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["cat"], 2);
        assert_eq!(counts["dog"], 1);
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn empty_and_punctuation_only_text_has_no_words() {
        assert!(count_words("").is_empty());
        assert!(count_words("  ... !!! \n").is_empty());
    }

    #[test]
    fn apostrophes_stay_inside_words() {
        let counts = count_words("Don't 'quote' don't");
        assert_eq!(counts["don't"], 2);
        assert_eq!(counts["quote"], 1);
    }

    #[test]
    fn accented_letters_are_letters() {
        let counts = count_words("Café café CAFÉ");
        assert_eq!(counts["café"], 3);
    }

    #[test]
    fn ties_come_out_in_code_point_order() {
        let counts = count_words("b a c a b");
        let top = top_n(counts, 3);
        let expected = vec![
            ("a".to_string(), 2),
            ("b".to_string(), 2),
            ("c".to_string(), 1),
        ];
        assert_eq!(top, expected);
    }

    #[test]
    fn accented_words_sort_after_plain_ones() {
        let top = top_n(count_words("école zoo"), 2);
        assert_eq!(top[0].0, "zoo");
        assert_eq!(top[1].0, "école");
    }

    #[test]
    fn top_n_keeps_only_n() {
        let counts = count_words("a b c d");
        assert_eq!(top_n(counts, 2).len(), 2);
    }
}
