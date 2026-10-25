use std::fmt;
use std::io;

pub enum CliError {
    /// The command line is wrong. Exit code 2.
    Usage(String),
    /// The input could not be read. Exit code 1.
    Read { what: String, cause: io::Error },
}

impl CliError {
    pub fn exit_code(&self) -> u8 {
        match self {
            CliError::Usage(_) => 2,
            CliError::Read { .. } => 1,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Usage(message) => write!(f, "{message}"),
            CliError::Read { what, cause } => write!(f, "cannot read {what}: {cause}"),
        }
    }
}
