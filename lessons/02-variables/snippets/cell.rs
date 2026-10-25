use std::cell::Cell;

fn main() {
    let counter = Cell::new(0); // no `mut`
    counter.set(counter.get() + 1);
    println!("counter: {}", counter.get());
}
