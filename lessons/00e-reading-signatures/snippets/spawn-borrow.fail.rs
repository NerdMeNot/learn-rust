use std::thread;

fn main() {
    let name = String::from("Ferris");
    let handle = thread::spawn(|| {
        println!("hello, {name}");
    });
    handle.join().unwrap();
}
