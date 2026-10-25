use std::str::FromStr;

mod port {
    use std::fmt;
    use std::str::FromStr;

    /// A TCP port from 1 to 65535. The only way to get one is `Port::try_from` or `parse`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Port(u16);

    #[derive(Debug, PartialEq)]
    pub enum PortError {
        NotANumber,
        OutOfRange(u32),
    }

    impl fmt::Display for PortError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                PortError::NotANumber => write!(f, "not a number"),
                PortError::OutOfRange(n) => write!(f, "{n} is outside 1-65535"),
            }
        }
    }

    impl TryFrom<u32> for Port {
        type Error = PortError;

        fn try_from(n: u32) -> Result<Port, PortError> {
            match u16::try_from(n) {
                Ok(p) if p != 0 => Ok(Port(p)),
                _ => Err(PortError::OutOfRange(n)),
            }
        }
    }

    impl FromStr for Port {
        type Err = PortError;

        fn from_str(s: &str) -> Result<Port, PortError> {
            let n: u32 = s.trim().parse().map_err(|_| PortError::NotANumber)?;
            Port::try_from(n)
        }
    }

    impl Port {
        pub fn get(self) -> u16 {
            self.0
        }
    }
}

use port::Port;

/// No check here: a `Port` is already known to be valid.
fn listen(port: Port) -> String {
    format!("listening on {}", port.get())
}

fn main() {
    for text in ["8080", "0", "70000", "http"] {
        match Port::from_str(text) {
            Ok(port) => println!("{text:>6} -> {}", listen(port)),
            Err(e) => println!("{text:>6} -> rejected: {e}"),
        }
    }
}
