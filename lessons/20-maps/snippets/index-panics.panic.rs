use std::collections::HashMap;

fn main() {
    let prices = HashMap::from([("tea", 3), ("milk", 2)]);
    println!("tea: {}", prices["tea"]);
    println!("coffee: {}", prices["coffee"]);
}
