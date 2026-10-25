use std::rc::Rc;

struct Counter {
    hits: u32,
}

fn main() {
    let a = Rc::new(Counter { hits: 0 });
    let b = Rc::clone(&a);

    a.hits += 1;
    b.hits += 1;
    println!("hits: {}", a.hits);
}
