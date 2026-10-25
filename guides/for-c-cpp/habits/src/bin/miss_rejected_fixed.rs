use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    map.entry(key).or_insert_with(|| format!("value {key}"))
}

fn main() {
    let mut map = HashMap::new();
    println!("{}", get_or_insert(&mut map, 1));
    println!("{}", get_or_insert(&mut map, 1));
}
