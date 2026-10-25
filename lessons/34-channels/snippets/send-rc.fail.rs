use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(Rc::new(5)).unwrap();

    let reader = thread::spawn(move || {
        println!("got {}", rx.recv().unwrap());
    });
    reader.join().unwrap();
}
