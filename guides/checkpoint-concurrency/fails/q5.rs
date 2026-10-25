use std::thread;

fn main() {
    let name = String::from("worker");
    let handle = thread::spawn(|| println!("hello from {name}"));
    handle.join().unwrap();
}
