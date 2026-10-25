use std::rc::Rc;

struct Counter {
    hits: u32,
}

fn main() {
    let only = Rc::new(Counter { hits: 0 }); // one owner, and still read-only
    only.hits += 1;
}
