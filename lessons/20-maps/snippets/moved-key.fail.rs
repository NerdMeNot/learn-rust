use std::collections::HashMap;

fn main() {
    let name = String::from("ana");
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(name, 30);
    println!("inserted {name}");
}
