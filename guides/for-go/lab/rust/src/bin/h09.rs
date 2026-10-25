use std::sync::Mutex;
use std::thread;

fn main() {
    let count = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    *count.lock().unwrap() += 1;
                }
            });
        }
    });
    println!("{}", count.into_inner().unwrap());
}
