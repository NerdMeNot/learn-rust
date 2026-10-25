use std::collections::HashMap;

fn main() {
    let mut by_length = HashMap::new();
    by_length.entry(3).or_default().push("fig");
    println!("{}", by_length.len());
}
