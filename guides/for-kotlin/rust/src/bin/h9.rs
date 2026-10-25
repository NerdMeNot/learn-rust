use std::collections::HashMap;

struct Server {
    host: String,
    port: u16,
    tls: bool,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
        }
    }
}

fn main() {
    let s = Server {
        port: 9000,
        ..Default::default()
    };
    println!("{}:{} tls={}", s.host, s.port, s.tls);
    let cfg = {
        let mut m = HashMap::new();
        m.insert("a", "1");
        m
    };
    println!("{cfg:?}");
    let next = "41".parse::<i32>().ok().map(|n| n + 1);
    println!("{next:?}");
}
