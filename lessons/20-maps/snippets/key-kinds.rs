use std::collections::HashMap;

fn main() {
    let text = String::from("red green red");

    // &str keys point into `text`: nothing is copied, but `text` must outlive the map.
    let mut borrowed: HashMap<&str, u32> = HashMap::new();
    for word in text.split(' ') {
        *borrowed.entry(word).or_insert(0) += 1;
    }

    // String keys are copies: the map owns them.
    let mut owned: HashMap<String, u32> = HashMap::new();
    for word in text.split(' ') {
        *owned.entry(word.to_string()).or_insert(0) += 1;
    }

    println!("borrowed red: {:?}", borrowed.get("red"));
    println!("owned red: {:?}", owned.get("red"));
}
