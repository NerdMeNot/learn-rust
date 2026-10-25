use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // tx sends, rx receives

    thread::spawn(move || {
        for i in 1..=3 {
            let msg = format!("job {i}");
            tx.send(msg).unwrap(); // msg moves into the channel
        }
    }); // tx goes away here: channel closes

    // ends when the channel closes
    for msg in rx {
        println!("got {msg}");
    }
}
