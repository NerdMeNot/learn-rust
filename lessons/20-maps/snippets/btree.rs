use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("cy", 70);
    scores.insert("ana", 91);
    scores.insert("dee", 60);
    scores.insert("bo", 85);

    println!("all: {scores:?}");
    println!("first: {:?}", scores.first_key_value());
    println!("last: {:?}", scores.last_key_value());
    for (name, score) in scores.range("b".."d") {
        println!("from b up to (not including) d: {name} {score}");
    }
    // Like Java's floorKey and ceilingKey:
    println!(
        "last key at or below \"c\": {:?}",
        scores.range(..="c").next_back()
    );
    println!(
        "first key at or after \"c\": {:?}",
        scores.range("c"..).next()
    );
}
