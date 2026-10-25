// Most closures you write call a method that takes one: `retain`, `sort_by_key`.
// Here both use a local variable from the function around them.

// Removes the words in `banned`, then sorts the rest by how far their length (in
// bytes) is from `target`, nearest first. Words the same distance away stay in
// alphabetical order.
fn closest(words: &mut Vec<String>, target: usize, banned: &[&str]) {
    words.retain(|w| !banned.contains(&w.as_str()));
    words.sort_by_key(|w| (w.len().abs_diff(target), w.clone()));
}

fn main() {
    let mut words: Vec<String> = ["pear", "fig", "banana", "kiwi", "apple", "plum"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    closest(&mut words, 4, &["apple"]);
    // distance 0: kiwi, pear, plum (alphabetical); distance 1: fig; distance 2: banana
    assert_eq!(words, ["kiwi", "pear", "plum", "fig", "banana"]);

    // A different target changes the order; a banned word that is not there is fine.
    closest(&mut words, 6, &["grape"]);
    assert_eq!(words, ["banana", "kiwi", "pear", "plum", "fig"]);

    // Nothing left, nothing to sort.
    let mut few = vec![String::from("fig")];
    closest(&mut few, 3, &["fig"]);
    assert!(few.is_empty());
    println!("all checks passed");
}
