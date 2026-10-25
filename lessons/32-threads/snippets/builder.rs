use std::thread::{self, Builder};

fn main() {
    println!("main thread name: {:?}", thread::current().name());

    let handle = Builder::new()
        .name("indexer".to_string())
        .stack_size(256 * 1024)
        .spawn(|| {
            // a thread can ask for its own name
            let me = thread::current();
            format!("hello from {:?}", me.name())
        })
        .expect("the OS refused to create the thread");
    println!("{}", handle.join().unwrap());

    let unnamed = thread::spawn(|| thread::current().name().map(String::from));
    println!("a plain spawn is named: {:?}", unnamed.join().unwrap());
}
