use std::thread;

fn main() {
    let mut results = Vec::new();
    let mut handles = Vec::new();
    for i in 0..3 {
        handles.push(thread::spawn(move || {
            results.push(i * 10);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{results:?}");
}
