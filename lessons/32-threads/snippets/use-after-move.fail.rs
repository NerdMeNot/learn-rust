use std::thread;

fn main() {
    let names = vec![String::from("a"), String::from("b")];
    let handle = thread::spawn(move || names.len());
    println!("main still has {} names", names.len());
    handle.join().unwrap();
}
