use std::collections::HashMap;
use std::thread;

fn main() {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    thread::scope(|s| {
        s.spawn(|| counts.insert(1, 1));
        s.spawn(|| counts.insert(2, 2));
    });
}
