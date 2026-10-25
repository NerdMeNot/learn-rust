#[derive(Debug)]
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
    // `..base` takes only `verbose` here, which is Copy: `base` stays whole.
    let test = Config {
        name: "test".to_string(),
        port: 9000,
        ..base
    };
    println!("{} {} {}", base.name, base.port, base.verbose);
    println!("{} {} {}", test.name, test.port, test.verbose);
}
