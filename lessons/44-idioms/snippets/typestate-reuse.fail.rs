use std::marker::PhantomData;

struct Closed;
struct Open;

struct Connection<State> {
    state: PhantomData<State>,
}

impl Connection<Closed> {
    fn new() -> Connection<Closed> {
        Connection { state: PhantomData }
    }

    fn connect(self) -> Connection<Open> {
        Connection { state: PhantomData }
    }
}

fn main() {
    let closed = Connection::new();
    let _open = closed.connect();
    let _again = closed.connect();
}
