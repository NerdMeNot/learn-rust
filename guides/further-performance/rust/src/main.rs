use std::collections::HashMap;

const TEXT: &str = "the quick brown fox jumps over the lazy dog the end \
    a rust program counts words and prints the most common ones first";

fn main() {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in TEXT.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut ranked: Vec<(&str, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (word, n) in ranked.iter().take(3) {
        println!("{word:>6}: {n}");
    }
    println!("{} distinct words", ranked.len());
}
