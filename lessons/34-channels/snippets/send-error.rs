use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(String::from("first")).unwrap(); // the receiver still exists
    drop(rx);

    let err = tx.send(String::from("second")).unwrap_err();
    println!("error: {err}");
    println!("the value comes back: {:?}", err.0);

    // A producer can use this to notice that nobody is listening and stop.
    let mut sent = 0;
    for i in 0..5 {
        if tx.send(i.to_string()).is_err() {
            break;
        }
        sent += 1;
    }
    println!("sent after the receiver left: {sent}");
}
