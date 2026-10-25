use std::collections::HashMap;

fn counter<'a>(map: &'a mut HashMap<String, u32>, key: &str) -> &'a mut u32 {
    map.entry(key.to_string()).or_default()
}

fn main() {
    let mut map = HashMap::new();
    *counter(&mut map, "pear") += 1;
    *counter(&mut map, "pear") += 1;
    println!("{map:?}");
}
