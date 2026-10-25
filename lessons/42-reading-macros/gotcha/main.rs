use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

fn main() {
    let ages = hashmap! { "Ada": 36, "Linus": 28 };
    println!("entries: {}", ages.len());
}
