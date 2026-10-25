use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    println!("insert apple 3: {:?}", stock.insert("apple".to_string(), 3));
    println!("insert pear 5: {:?}", stock.insert("pear".to_string(), 5));
    println!("insert apple 4: {:?}", stock.insert("apple".to_string(), 4));
    println!("len: {}", stock.len());

    println!("get apple: {:?}", stock.get("apple"));
    println!("get plum: {:?}", stock.get("plum"));
    println!("contains_key pear: {}", stock.contains_key("pear"));

    if let Some(n) = stock.get_mut("pear") {
        *n += 10;
    }
    println!("pear after get_mut: {:?}", stock.get("pear"));

    println!("remove pear: {:?}", stock.remove("pear"));
    println!("remove pear again: {:?}", stock.remove("pear"));
    println!("len: {}", stock.len());

    let prices = HashMap::from([("tea", 3), ("milk", 2)]);
    println!("prices[\"tea\"]: {}", prices["tea"]);
}
