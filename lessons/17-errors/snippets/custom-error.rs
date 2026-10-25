use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// One enum lists every way reading the setting can fail.
#[derive(Debug)]
enum ConfigError {
    Missing,
    NotANumber(ParseIntError),
    OutOfRange { value: u32, max: u32 },
}

// Display is the message a person reads. It describes this level only.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Missing => write!(f, "the threads setting is missing"),
            ConfigError::NotANumber(_) => write!(f, "the threads setting is not a number"),
            ConfigError::OutOfRange { value, max } => {
                write!(f, "threads is {value}, but the most allowed is {max}")
            }
        }
    }
}

// Error needs Debug and Display. source says which error caused this one.
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::NotANumber(cause) => Some(cause),
            _ => None,
        }
    }
}

// This is the conversion that ? calls when it finds a ParseIntError in a
// function that returns ConfigError.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> ConfigError {
        ConfigError::NotANumber(e)
    }
}

fn threads(setting: Option<&str>) -> Result<u32, ConfigError> {
    let text = setting.ok_or(ConfigError::Missing)?;
    let n: u32 = text.parse()?;
    if n > 64 {
        return Err(ConfigError::OutOfRange { value: n, max: 64 });
    }
    Ok(n)
}

fn main() {
    for setting in [Some("8"), None, Some("many"), Some("500")] {
        match threads(setting) {
            Ok(n) => println!("ok: {n} threads"),
            Err(e) => {
                println!("Display: {e}");
                println!("Debug:   {e:?}");
                if let Some(cause) = e.source() {
                    println!("cause:   {cause}");
                }
            }
        }
    }
}
