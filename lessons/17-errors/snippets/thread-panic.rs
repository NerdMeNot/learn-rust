use std::thread;

// A thread is a second line of execution (a later lesson). Read the closure
// as "run this on another thread".
fn main() {
    let worker = thread::spawn(|| {
        panic!("the worker hit a bug");
    });
    let outcome = worker.join();
    println!("the worker panicked: {}", outcome.is_err());
    println!("main is still running");
}
