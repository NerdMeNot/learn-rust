use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<&str, u32> = HashMap::new();
    ages.insert("ana", 30);
    let age: u32 = ages.get("ana");
    println!("{age}");
}
