use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("in the thread");
        6 * 7
    });
    // join waits for the thread, then hands back what the closure returned
    let result = handle.join();
    println!("join gave: {:?}", result);
}
