//! Configuration from the environment.

use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub bind: SocketAddr,
    pub database_url: String,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    #[error("BIND_ADDR must look like 127.0.0.1:3000, got {0:?}")]
    BadBind(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Reads settings through `get`, so a test can pass a closure instead of
    /// changing the real environment (which is `unsafe` to do in edition 2024).
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let bind = get("BIND_ADDR").unwrap_or_else(|| "127.0.0.1:3000".to_string());
        let bind = bind.parse().map_err(|_| ConfigError::BadBind(bind))?;
        let database_url =
            get("DATABASE_URL").unwrap_or_else(|| "sqlite://notes.db?mode=rwc".to_string());
        Ok(Config { bind, database_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name| map.get(name).cloned()
    }

    #[test]
    fn defaults_when_nothing_is_set() {
        let config = Config::from_lookup(lookup(&[])).unwrap();
        assert_eq!(config.bind, "127.0.0.1:3000".parse().unwrap());
        assert_eq!(config.database_url, "sqlite://notes.db?mode=rwc");
    }

    #[test]
    fn reads_both_settings() {
        let config = Config::from_lookup(lookup(&[
            ("BIND_ADDR", "0.0.0.0:8080"),
            ("DATABASE_URL", "sqlite::memory:"),
        ]))
        .unwrap();
        assert_eq!(config.bind.port(), 8080);
        assert_eq!(config.database_url, "sqlite::memory:");
    }

    #[test]
    fn names_the_setting_that_is_wrong() {
        let err = Config::from_lookup(lookup(&[("BIND_ADDR", "nope")])).unwrap_err();
        assert_eq!(err, ConfigError::BadBind("nope".to_string()));
    }
}
