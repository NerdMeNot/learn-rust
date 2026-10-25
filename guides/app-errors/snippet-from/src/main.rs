use anyhow::Context;
use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read the file")]
    Read(#[from] io::Error),
    #[error("port is not a number")]
    Parse(#[from] ParseIntError),
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let text = fs::read_to_string(path)?; // io::Error -> ConfigError::Read
    let port = text.trim().parse()?; // ParseIntError -> ConfigError::Parse
    Ok(port)
}

fn main() {
    fs::write("bad.conf", "eighty\n").unwrap();

    for path in ["bad.conf", "missing.conf"] {
        let e = read_port(path)
            .with_context(|| format!("loading {path}"))
            .unwrap_err();
        println!("{e:#}");

        // The ConfigError is still inside, so a caller can match on it.
        match e.downcast_ref::<ConfigError>() {
            Some(ConfigError::Read(io)) => println!("  io error kind: {:?}", io.kind()),
            Some(ConfigError::Parse(_)) => println!("  not a number"),
            None => println!("  something else"),
        }
    }
}
