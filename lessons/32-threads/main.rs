use std::thread;

fn main() {
    let names = vec!["a", "b", "c"];

    let handle = thread::spawn(move || {     // move: the thread takes names
        for n in &names {
            println!("worker sees {n}");
        }
        names.len()                          // threads can return a value
    });

    let count = handle.join().unwrap();      // wait for the thread
    println!("worker counted {count}");
}
