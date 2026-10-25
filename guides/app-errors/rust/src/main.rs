use anyhow::Context;
use std::fs;
use thiserror::Error;

// Library-style error: callers can match on each case.
#[derive(Debug, Error)]
enum ConfigError {
    #[error("config file not found: {0}")]
    Missing(String),
    #[error("port must be a number, got {0:?}")]
    BadPort(String),
}

fn read_config(path: &str) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|_| ConfigError::Missing(path.to_string()))
}

fn parse_port(text: &str) -> Result<u16, ConfigError> {
    let text = text.trim();
    text.parse()
        .map_err(|_| ConfigError::BadPort(text.to_string()))
}

// Application code: anyhow adds context, like Go's fmt.Errorf("...: %w", err).
fn load_port(path: &str) -> anyhow::Result<u16> {
    let text = read_config(path).context("loading settings")?;
    let port = parse_port(&text).with_context(|| format!("reading {path}"))?;
    Ok(port)
}

fn main() {
    fs::write("good.conf", "8080\n").unwrap();
    fs::write("bad.conf", "eighty\n").unwrap();

    for path in ["good.conf", "bad.conf", "missing.conf"] {
        match load_port(path) {
            Ok(port) => println!("{path}: port {port}"),
            Err(e) => println!("{path}: {e:#}"), // {:#} prints the whole chain
        }
    }

    // Look for a specific error type held by this anyhow::Error (downcast_ref).
    if let Err(e) = load_port("bad.conf") {
        if let Some(ConfigError::BadPort(value)) = e.downcast_ref::<ConfigError>() {
            println!("the bad value was {value:?}");
        }
    }
}
