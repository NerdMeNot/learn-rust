mod port {
    /// A port from 1 to 65535. The field is private: only `parse` can build one.
    pub struct Port(u16);

    impl Port {
        pub fn parse(text: &str) -> Result<Port, String> {
            let n: u32 = text.parse().map_err(|_| "not a number".to_string())?;
            if n == 0 || n > 65535 {
                return Err(format!("{n} is outside 1-65535"));
            }
            Ok(Port(n as u16))
        }

        pub fn get(&self) -> u16 {
            self.0
        }
    }
}

use port::Port;

// A Port is already valid, so this function has nothing to check.
fn listen(port: &Port) {
    println!("listening on {}", port.get());
}

fn main() {
    for text in ["8080", "0", "http"] {
        match Port::parse(text) {
            Ok(port) => listen(&port),
            Err(e) => println!("{text:?}: {e}"),
        }
    }
}
