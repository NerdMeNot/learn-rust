use std::fmt::{self, Display, Formatter};

struct Plain(u32);
struct Padded(u32);
struct Forward(f64);

impl Display for Plain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{}", self.0) // a new, empty format spec: the caller's is dropped
    }
}

impl Display for Padded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(&format!("#{}", self.0)) // applies width, fill, alignment and precision
    }
}

impl Display for Forward {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f) // hands the caller's spec to the f64
    }
}

fn main() {
    println!("write!:   [{:>8}] [{:<8}]", Plain(42), Plain(42));
    println!(
        "pad:      [{:>8}] [{:<8}] [{:*^8}]",
        Padded(42),
        Padded(42),
        Padded(42)
    );
    println!("pad, .2:  [{:.2}]", Padded(12345));
    println!(
        "forward:  [{:8.2}] [{:+.1}]",
        Forward(3.14159),
        Forward(3.14159)
    );
    println!("to_string: [{}]", Padded(42).to_string());
}
