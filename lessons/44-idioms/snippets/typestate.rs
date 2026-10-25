use std::marker::PhantomData;
use std::mem::size_of;

struct Closed;
struct Open;

struct Connection<State> {
    host: String,
    sent: u32,
    state: PhantomData<State>,
}

impl Connection<Closed> {
    fn new(host: &str) -> Connection<Closed> {
        Connection {
            host: host.to_string(),
            sent: 0,
            state: PhantomData,
        }
    }

    /// Takes `self` by value: the closed connection is gone, an open one comes back.
    fn connect(self) -> Connection<Open> {
        println!("connecting to {}", self.host);
        Connection {
            host: self.host,
            sent: self.sent,
            state: PhantomData,
        }
    }
}

impl Connection<Open> {
    fn send(&mut self, text: &str) {
        self.sent += 1;
        println!("sent {text:?} (message {})", self.sent);
    }

    fn close(self) -> Connection<Closed> {
        println!("closing after {} messages", self.sent);
        Connection {
            host: self.host,
            sent: self.sent,
            state: PhantomData,
        }
    }
}

fn main() {
    let conn = Connection::new("example.org");
    let mut conn = conn.connect();
    conn.send("hello");
    conn.send("world");
    let conn = conn.close();

    println!(
        "size_of::<Connection<Closed>>(): {}",
        size_of::<Connection<Closed>>()
    );
    println!(
        "size_of::<Connection<Open>>():   {}",
        size_of::<Connection<Open>>()
    );
    println!(
        "size_of::<(String, u32)>():      {}",
        size_of::<(String, u32)>()
    );
    println!("host is still {}", conn.host);
}
