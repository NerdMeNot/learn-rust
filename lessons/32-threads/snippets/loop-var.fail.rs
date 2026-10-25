use std::thread;

fn main() {
    let mut handles = Vec::new();
    for i in 0..3 {
        handles.push(thread::spawn(|| println!("thread {i}")));
    }
    for h in handles {
        h.join().unwrap();
    }
}
