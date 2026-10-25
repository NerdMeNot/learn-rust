// A `Copy` type: `self` copies it, like a Go value receiver.
#[derive(Clone, Copy)]
struct Counter {
    count: u32,
}

impl Counter {
    fn bump_copy(mut self) {
        self.count += 1;
    }

    fn bump_in_place(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    c.bump_copy();
    println!("after bump_copy: {}", c.count);
    c.bump_in_place();
    println!("after bump_in_place: {}", c.count);
}
