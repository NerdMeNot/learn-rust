fn port(text: &str) -> u16 {
    text.parse()
}

fn main() {
    println!("{}", port("8080"));
}
