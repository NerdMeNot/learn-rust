use std::thread;

fn main() {
    let handle = thread::spawn(|| -> i32 { panic!("worker failed") });
    let n = handle.join().unwrap();
    println!("never reached: {n}");
}
