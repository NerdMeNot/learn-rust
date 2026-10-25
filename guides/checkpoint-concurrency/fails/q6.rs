use std::sync::Arc;
use std::thread;

fn main() {
    let list = Arc::new(Vec::new());
    let mut handles = Vec::new();
    for i in 0..3 {
        let list = Arc::clone(&list);
        handles.push(thread::spawn(move || list.push(i)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", list.len());
}
