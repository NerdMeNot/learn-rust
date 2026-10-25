use std::rc::Rc;
use std::thread;

fn main() {
    let numbers = Rc::new(vec![1, 2, 3]);
    let for_thread = Rc::clone(&numbers);
    let handle = thread::spawn(move || println!("in the thread: {:?}", for_thread));
    handle.join().unwrap();
    println!("in main: {:?}", numbers);
}
