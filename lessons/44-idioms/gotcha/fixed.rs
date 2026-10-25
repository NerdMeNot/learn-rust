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
    // `..base.clone()` hands over a copy, so `base` stays whole.
    let test = Config {
        port: 9000,
        ..base.clone()
    };
    println!("{base:?}");
    println!("{test:?}");
    println!(
        "{} on port {}, verbose: {}",
        test.name, test.port, test.verbose
    );
}
