mod config {
    /// Reads a port number such as "8080".
    pub fn parse_port(text: &str) -> u16 {
        text.trim().parse().unwrap()
    }
}

fn main() {
    for input in ["8080", "80a", "70000"] {
        println!("{input:?} -> {}", config::parse_port(input));
    }
}
