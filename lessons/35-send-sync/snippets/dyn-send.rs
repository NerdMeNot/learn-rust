use std::thread;

fn run(job: Box<dyn Fn() -> i32 + Send>) -> i32 {
    job()
}

fn main() {
    let job: Box<dyn Fn() -> i32 + Send> = Box::new(|| 6 * 7);
    let handle = thread::spawn(move || run(job));
    println!("job gave {}", handle.join().unwrap());
}
