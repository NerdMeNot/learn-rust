use std::cell::Cell;
use std::thread;

fn main() {
    let hits = Cell::new(0);
    thread::scope(|s| {
        s.spawn(|| hits.set(hits.get() + 1)); // borrows `hits`: no `move`, no 'static needed
    });
    println!("hits: {}", hits.get());
}
