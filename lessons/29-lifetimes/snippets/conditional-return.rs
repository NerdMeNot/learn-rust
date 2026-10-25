use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    map.entry(key).or_insert_with(|| String::from("new"))
}

fn main() {
    let mut map = HashMap::new();
    println!("first call: {}", get_or_insert(&mut map, 1));
    println!("second call: {}", get_or_insert(&mut map, 1));
    println!("entries: {}", map.len());
}
