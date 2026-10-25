use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    println!("owners at the start: {}", Arc::strong_count(&data));

    let copies: Vec<Arc<Vec<i32>>> = (0..3).map(|_| Arc::clone(&data)).collect();
    println!("owners after three clones: {}", Arc::strong_count(&data));
    println!("same Vec: {}", Arc::ptr_eq(&data, &copies[0]));

    let handles: Vec<_> = copies
        .into_iter()
        .map(|mine| thread::spawn(move || mine.iter().sum::<i32>()))
        .collect();
    for h in handles {
        println!("a thread summed: {}", h.join().unwrap());
    }
    println!("owners after the joins: {}", Arc::strong_count(&data));
}
