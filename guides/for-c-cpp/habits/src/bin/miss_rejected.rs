use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    if let Some(found) = map.get(&key) {
        return found;
    }
    map.insert(key, format!("value {key}"));
    &map[&key]
}

fn main() {
    let mut map = HashMap::new();
    println!("{}", get_or_insert(&mut map, 1));
}
