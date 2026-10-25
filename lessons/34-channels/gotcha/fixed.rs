use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut msg = vec![1, 2];
        msg.push(3); // finish building it first
        tx.send(msg).unwrap(); // then hand it over
    });
    println!("{:?}", rx.recv().unwrap());
}
