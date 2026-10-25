use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let report = String::from("done");
        tx.send(report).unwrap();
        println!("sent {report}");
    });
    println!("{}", rx.recv().unwrap());
    handle.join().unwrap();
}
