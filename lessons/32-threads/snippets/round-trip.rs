use std::thread;

fn main() {
    let numbers = vec![42, 7, 19, 3];
    let before = numbers.as_ptr();

    // move the Vec in, work on it, return it: ownership goes there and back
    let handle = thread::spawn(move || {
        let mut numbers = numbers;
        numbers.sort();
        numbers
    });
    let sorted = handle.join().unwrap();

    println!("sorted: {:?}", sorted);
    println!("same buffer, nothing copied: {}", sorted.as_ptr() == before);
}
