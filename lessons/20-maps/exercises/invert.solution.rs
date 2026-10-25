use std::collections::{BTreeMap, HashMap};

/// Turn name -> score into score -> names, with the scores in increasing order.
/// Two names can share a score, so each score maps to a list. Every list is sorted.
fn invert(scores: &HashMap<String, u32>) -> BTreeMap<u32, Vec<String>> {
    let mut inverted: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for (name, score) in scores {
        inverted.entry(*score).or_default().push(name.clone());
    }
    for names in inverted.values_mut() {
        names.sort();
    }
    inverted
}

fn main() {
    assert!(invert(&HashMap::new()).is_empty());

    let scores = HashMap::from([
        ("cy".to_string(), 7),
        ("ana".to_string(), 7),
        ("bo".to_string(), 2),
        ("dee".to_string(), 90),
    ]);
    let inverted = invert(&scores);
    assert_eq!(inverted.len(), 3);
    assert_eq!(inverted[&7], vec!["ana", "cy"]);
    assert_eq!(inverted[&2], vec!["bo"]);
    assert_eq!(inverted.get(&3), None);

    // A BTreeMap loops in key order, so this is the same on every run.
    let order: Vec<u32> = inverted.keys().copied().collect();
    assert_eq!(order, vec![2, 7, 90]);
    assert_eq!(inverted.last_key_value().map(|(score, _)| *score), Some(90));

    println!("all checks passed");
}
