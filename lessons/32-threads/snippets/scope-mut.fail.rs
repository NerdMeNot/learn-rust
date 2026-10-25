use std::thread;

fn main() {
    let mut count = 0;
    thread::scope(|s| {
        s.spawn(|| count += 1);
        s.spawn(|| count += 1);
    });
    println!("count: {count}");
}
