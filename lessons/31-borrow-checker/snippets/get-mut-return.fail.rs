use std::collections::HashMap;

fn counter<'a>(map: &'a mut HashMap<String, u32>, key: &str) -> &'a mut u32 {
    if let Some(n) = map.get_mut(key) {
        return n;
    }
    map.insert(key.to_string(), 0);
    map.get_mut(key).unwrap()
}

fn main() {
    let mut map = HashMap::new();
    *counter(&mut map, "pear") += 1;
    *counter(&mut map, "pear") += 1;
    println!("{map:?}");
}
