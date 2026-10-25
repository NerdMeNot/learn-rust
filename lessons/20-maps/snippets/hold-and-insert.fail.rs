use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::from([("apple", 3)]);
    let apples = stock.get("apple");
    stock.insert("pear", 5);
    println!("apples: {apples:?}");
}
