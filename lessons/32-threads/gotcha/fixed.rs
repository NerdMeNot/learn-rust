use std::thread;

fn main() {
    let names = vec!["a", "b", "c"];
    let handle = thread::spawn(move || {
        println!("{}", names.len());
    });
    handle.join().unwrap();
}
