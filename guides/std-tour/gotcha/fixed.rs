use std::collections::HashMap;

fn main() {
    let names = vec!["ana", "bo"];
    let mut ages = HashMap::new();
    ages.insert("ana", 30);
    println!("{} {}", names.len(), ages.len());
}
