use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::from([("apple", 3), ("pear", 5)]);
    for name in stock.keys() {
        stock.insert("fig", name.len());
    }
}
