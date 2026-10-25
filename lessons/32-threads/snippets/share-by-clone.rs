use std::thread;

fn main() {
    let names = vec![String::from("a"), String::from("b")];

    // the thread gets its own copy; main keeps the original
    let for_thread = names.clone();
    let handle = thread::spawn(move || for_thread.len());

    println!("main still has {} names", names.len());
    println!("the thread counted {}", handle.join().unwrap());
}
