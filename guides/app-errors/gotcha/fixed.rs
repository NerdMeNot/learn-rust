use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("config file not found: {0}")]
    Missing(String),
}

fn read_config(path: &str) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|_| ConfigError::Missing(path.to_string()))
}

fn main() {
    match read_config("missing.conf") {
        Ok(text) => println!("{text}"),
        Err(e) => println!("{e}"),
    }
}
