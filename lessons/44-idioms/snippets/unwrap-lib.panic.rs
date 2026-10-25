/// A "library" function: it decides, for every caller, that bad input is fatal.
pub fn parse_port(text: &str) -> u16 {
    text.trim().parse().unwrap()
}

fn main() {
    println!("{}", parse_port("8080"));
    println!("{}", parse_port("http"));
}
