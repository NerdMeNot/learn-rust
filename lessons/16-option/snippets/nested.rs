use std::collections::HashMap;

fn main() {
    // A map from item name to a count that may itself be unknown.
    let mut stock: HashMap<&str, Option<u32>> = HashMap::new();
    stock.insert("apple", Some(3));
    stock.insert("pear", None);

    // One lookup keeps "the key holds None" apart from "no such key".
    println!("get apple: {:?}", stock.get("apple"));
    println!("get pear: {:?}", stock.get("pear"));
    println!("get plum: {:?}", stock.get("plum"));
}
