use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::from([("apple", 3), ("pear", 0)]);

    // extend inserts every pair; a key that is already there gets the new value.
    stock.extend([("apple", 9), ("fig", 2), ("kiwi", 0)]);
    println!(
        "after extend: len {}, apple {:?}",
        stock.len(),
        stock.get("apple")
    );

    // retain keeps the pairs for which the function returns true.
    stock.retain(|_name, count| *count > 0);
    println!("after retain: len {}", stock.len());

    // drain hands out every pair and leaves the map empty.
    let mut taken: Vec<_> = stock.drain().collect();
    taken.sort();
    println!("drained: {taken:?}");
    println!("after drain: len {}", stock.len());
}
