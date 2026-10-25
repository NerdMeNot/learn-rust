use std::thread;

fn main() {
    let n = thread::available_parallelism().expect("the OS would not say");
    // the number belongs to the machine running this page: only a bound is stable
    println!("at least one: {}", n.get() >= 1);
}
