use std::sync::mpsc;

fn main() {
    let (_tx, rx) = mpsc::channel::<i32>();
    let _second = rx.clone();
}
