use std::thread;

// `spawn` needs `F: FnOnce() -> T + Send + 'static`: the closure may not borrow `numbers`.
fn sum_in_thread(numbers: Vec<i32>) -> i32 {
    todo!("spawn a `move` closure that sums `numbers`, then `join()` it; `join` gives a Result")
}

fn main() {
    assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
    assert_eq!(sum_in_thread(vec![]), 0);
    assert_eq!(sum_in_thread((1..=100).collect()), 5050);
    println!("all checks passed");
}
