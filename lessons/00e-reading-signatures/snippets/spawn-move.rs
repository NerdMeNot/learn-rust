use std::thread;

fn main() {
    let name = String::from("Ferris");
    // `move` makes the closure own `name` instead of borrowing it
    let handle = thread::spawn(move || name.len());
    println!("joined: {:?}", handle.join());
}
