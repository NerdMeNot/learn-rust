use std::collections::HashMap;

// A macro: a pattern on the left of `=>`, the code it becomes on the right.
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

fn main() {
    let ages = hashmap! { "Ada" => 36, "Linus" => 28, "Grace" => 45 };

    let mut names: Vec<_> = ages.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {}", ages[name]);
    }
    println!("entries: {}", ages.len());
}
