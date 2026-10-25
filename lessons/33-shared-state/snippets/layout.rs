use std::sync::{Arc, Mutex};

fn main() {
    let shared = Arc::new(Mutex::new(vec![1, 2, 3]));
    let first = Arc::clone(&shared);
    let second = Arc::clone(&shared);

    println!("strong_count: {}", Arc::strong_count(&shared));
    println!("weak_count: {}", Arc::weak_count(&shared));
    println!("same allocation: {}", Arc::ptr_eq(&first, &second));
    let first_buffer = first.lock().unwrap().as_ptr();
    let second_buffer = second.lock().unwrap().as_ptr();
    println!("same buffer: {}", first_buffer == second_buffer);
    let (len, capacity) = {
        let guard = shared.lock().unwrap();
        (guard.len(), guard.capacity())
    };
    println!("Vec length: {len}, capacity: {capacity}");
    println!(
        "size of an Arc: {} bytes",
        size_of::<Arc<Mutex<Vec<i32>>>>()
    );
    println!("size of a Vec<i32>: {} bytes", size_of::<Vec<i32>>());
    println!(
        "size of a Mutex<Vec<i32>>: {} bytes",
        size_of::<Mutex<Vec<i32>>>()
    );
    println!("size of a Mutex<()>: {} bytes", size_of::<Mutex<()>>());
}
