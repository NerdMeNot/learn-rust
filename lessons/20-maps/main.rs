use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("ana"), 30);
    ages.insert(String::from("bo"), 25);

    // get returns Option: Some(&value) or None
    match ages.get("ana") {
        Some(age) => println!("ana is {age}"),
        None => println!("no ana"),
    }

    // a default on purpose: 0 when the key is missing
    let zed = ages.get("zed").copied().unwrap_or(0);
    println!("zed is {zed}");

    // count words: entry() means "get it, or insert it first"
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in "a b a c a".split(' ') {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("a appears {} times", counts["a"]);
}
