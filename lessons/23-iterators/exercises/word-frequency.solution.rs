use std::collections::HashMap;

/// The `n` most common words in `text`, most common first, as (word, count).
/// A word is a run of letters (`char::is_alphabetic`), compared in lowercase.
/// Words with the same count are sorted alphabetically.
fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
    {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.into_iter().take(n).collect()
}

fn main() {
    // Ties are broken alphabetically: "and" comes before "cat" and "hat".
    assert_eq!(
        top_words("the cat and the hat", 2),
        vec![("the".to_string(), 2), ("and".to_string(), 1)]
    );
    // Case and punctuation do not make a different word.
    assert_eq!(
        top_words("Go, go! GO? rust.", 1),
        vec![("go".to_string(), 3)]
    );
    // Letters outside English are letters too.
    assert_eq!(
        top_words("Ölçek ölçek straße", 5),
        vec![("ölçek".to_string(), 2), ("straße".to_string(), 1)]
    );
    // No words at all, not one empty word.
    assert_eq!(top_words("", 3), vec![]);
    assert_eq!(top_words("  ... !!! ", 3), vec![]);
    println!("all checks passed");
}
