use std::sync::mpsc;
use std::thread;

fn main() {
    std::panic::set_hook(Box::new(|_| {})); // keep the panic message out of this demo

    let (tx, rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        panic!("worker failed"); // unwinding drops tx
    });

    // The loop ends, because the panicked thread's sender was dropped like any other.
    let received: Vec<i32> = rx.iter().collect();
    println!("received before the failure: {received:?}");
    println!("worker.join() returned Err: {}", worker.join().is_err());
}
