use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct SettingError {
    key: String,
    cause: ParseIntError,
}

impl fmt::Display for SettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad value for {}", self.key)
    }
}

impl Error for SettingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

#[derive(Debug)]
struct LoadError {
    file: String,
    cause: SettingError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not load {}", self.file)
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

// Rust's version of Go's errors.As: walk the chain and test each error.
// (Copy the signature as written; Box<dyn Error> is explained above.)
fn find_parse_error<'a>(err: &'a (dyn Error + 'static)) -> Option<&'a ParseIntError> {
    let mut current = Some(err);
    while let Some(e) = current {
        if let Some(found) = e.downcast_ref::<ParseIntError>() {
            return Some(found);
        }
        current = e.source();
    }
    None
}

fn main() {
    let err = LoadError {
        file: "limits.txt".to_string(),
        cause: SettingError {
            key: "threads".to_string(),
            cause: "12k".parse::<u32>().unwrap_err(),
        },
    };

    println!("top: {err}");
    let mut current = err.source();
    let mut depth = 1;
    while let Some(cause) = current {
        println!("{depth} level down: {cause}");
        current = cause.source();
        depth += 1;
    }

    println!(
        "top level is a ParseIntError: {}",
        (&err as &dyn Error)
            .downcast_ref::<ParseIntError>()
            .is_some()
    );
    println!("found one in the chain: {:?}", find_parse_error(&err));
}
