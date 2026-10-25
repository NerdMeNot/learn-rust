use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut msg = vec![1, 2];
        tx.send(msg).unwrap();
        msg.push(3);
    });
    println!("{:?}", rx.recv().unwrap());
}
