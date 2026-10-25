use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let _ = tx.is_disconnected(); // unstable on 1.99.1
    let _ = rx.is_disconnected();
}
