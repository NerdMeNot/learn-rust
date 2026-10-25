use std::thread;

fn main() {
    let name = String::from("ferris");
    let handle = thread::spawn(move || println!("hello from a thread, {name}"));
    handle.join().unwrap();
}
