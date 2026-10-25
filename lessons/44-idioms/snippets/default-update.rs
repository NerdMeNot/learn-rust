#[derive(Debug, Clone, PartialEq)]
struct Config {
    host: String,
    port: u16,
    retries: u32,
    verbose: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            retries: 3,
            verbose: false,
        }
    }
}

fn main() {
    let plain = Config::default();
    let custom = Config {
        port: 9000,
        verbose: true,
        ..Default::default()
    };
    println!("{plain:?}");
    println!("{custom:?}");
    assert_eq!(custom.host, plain.host);
}
