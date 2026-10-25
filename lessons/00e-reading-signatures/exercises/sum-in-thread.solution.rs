use std::thread;

// `spawn` needs `F: FnOnce() -> T + Send + 'static`: the closure may not borrow `numbers`.
fn sum_in_thread(numbers: Vec<i32>) -> i32 {
    let handle = thread::spawn(move || {
        let mut total = 0;
        for n in numbers {
            total += n;
        }
        total
    });
    handle.join().unwrap()
}

fn main() {
    assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
    assert_eq!(sum_in_thread(vec![]), 0);
    assert_eq!(sum_in_thread((1..=100).collect()), 5050);
    println!("all checks passed");
}
