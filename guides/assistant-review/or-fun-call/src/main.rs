use std::collections::HashMap;

fn load_from_disk(key: &str) -> String {
    println!("  (reading {key} from disk)");
    format!("default {key}")
}

fn main() {
    let mut cache = HashMap::new();
    cache.insert("theme", "dark".to_string());
    for key in ["theme", "theme", "font"] {
        let value = cache.get(key).cloned().unwrap_or(load_from_disk(key));
        println!("{key} = {value}");
    }
}
