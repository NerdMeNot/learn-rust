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

impl Connection<Open> {
    fn send(&mut self, text: &str) {
        println!("sent {text:?}");
    }
}

fn main() {
    let mut conn = Connection::new();
    conn.send("hello");
    let _open = conn.connect();
}
