use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    {
        let text = String::from("red green red");
        for word in text.split(' ') {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    println!("{}", counts.len());
}
