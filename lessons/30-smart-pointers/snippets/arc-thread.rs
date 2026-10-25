use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3]);
    let for_thread = Arc::clone(&numbers); // the same call, with an atomic count
    let handle = thread::spawn(move || println!("in the thread: {:?}", for_thread));
    handle.join().unwrap();
    println!("in main: {:?}", numbers);
    println!("owners: {}", Arc::strong_count(&numbers));
}
