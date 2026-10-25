use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("ana", 31);
    let ana: i32 = ages.get("ana");
    println!("ana: {ana}");
}
