use std::marker::PhantomData;

#[derive(Clone)]
struct Handle<T> {
    id: u32,
    _kind: PhantomData<T>,
}

struct Printer; // no Clone

fn main() {
    let h: Handle<Printer> = Handle {
        id: 1,
        _kind: PhantomData,
    };
    let again = h.clone();
    println!("{}", again.id);
}
