use std::cell::Cell;

static HITS: Cell<u32> = Cell::new(0);

fn main() {
    HITS.set(HITS.get() + 1);
    println!("hits: {}", HITS.get());
}
