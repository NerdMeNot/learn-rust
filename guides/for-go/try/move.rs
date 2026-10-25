use std::thread;

fn main() {
    let mut count = 0;
    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(move || {
                for _ in 0..1_000_000 {
                    count += 1;
                }
            });
        }
    });
    println!("{count}");
}
