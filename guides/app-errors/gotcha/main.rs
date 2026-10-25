use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("config file not found: {0}")]
    Missing(String),
}

fn read_config(path: &str) -> Result<String, ConfigError> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}

fn main() {
    match read_config("missing.conf") {
        Ok(text) => println!("{text}"),
        Err(e) => println!("{e}"),
    }
}
