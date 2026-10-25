use std::thread;

fn main() {
    let mut results = Vec::new();
    let mut handles = Vec::new();
    for i in 0..4 {
        handles.push(thread::spawn(move || {
            for _ in 0..100_000 {
                results.push(i * 10);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{} items", results.len());
}
