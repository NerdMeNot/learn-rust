use std::thread;

fn main() {
    let name = String::from("ferris");
    let handle = thread::spawn(|| println!("hello, {name}"));
    handle.join().unwrap();
}
