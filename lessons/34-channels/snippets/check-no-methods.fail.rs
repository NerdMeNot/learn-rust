use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let _ = rx.len();
    tx.close();
    let _ = tx.sender_count();
}
