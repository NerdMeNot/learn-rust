use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<u32, u32> = HashMap::new();
    println!("new: len {}, capacity {}", scores.len(), scores.capacity());

    let mut last = scores.capacity();
    for i in 0..40 {
        scores.insert(i, i * i);
        if scores.capacity() != last {
            println!(
                "len {}: capacity {} -> {}",
                scores.len(),
                last,
                scores.capacity()
            );
            last = scores.capacity();
        }
    }

    let roomy: HashMap<u32, u32> = HashMap::with_capacity(100);
    println!(
        "with_capacity(100): len {}, capacity >= 100: {}",
        roomy.len(),
        roomy.capacity() >= 100
    );
}
