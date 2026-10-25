use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    if let Some(value) = map.get(&key) {
        return value;
    }
    map.insert(key, String::from("new"));
    &map[&key]
}

fn main() {
    let mut map = HashMap::new();
    println!("value: {}", get_or_insert(&mut map, 1));
}
