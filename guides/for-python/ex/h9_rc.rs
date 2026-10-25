use std::rc::Rc;
use std::thread;

fn main() {
    let count = Rc::new(0);
    let handle = thread::spawn(move || println!("{count}"));
    handle.join().unwrap();
}
