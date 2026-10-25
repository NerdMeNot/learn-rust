#[derive(Debug, Clone)]
struct Config {
    name: String,
    port: u16,
    verbose: bool,
}

fn main() {
    let base = Config {
        name: "service".to_string(),
        port: 8080,
        verbose: false,
    };
    // "The same, but on another port."
    let test = Config { port: 9000, ..base };
    println!("{base:?}");
    println!("{test:?}");
}
