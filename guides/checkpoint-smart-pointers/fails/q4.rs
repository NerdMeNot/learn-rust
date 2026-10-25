use std::rc::Rc;
use std::thread;

fn main() {
    let numbers = Rc::new(vec![1, 2, 3]);
    let worker = {
        let numbers = Rc::clone(&numbers);
        thread::spawn(move || println!("{}", numbers.len()))
    };
    worker.join().unwrap();
    println!("{}", numbers.len());
}
