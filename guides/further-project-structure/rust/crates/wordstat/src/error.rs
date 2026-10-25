use std::fmt;

/// What can go wrong when counting words.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The text had no words in it.
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => f.write_str("no words to count"),
        }
    }
}

impl std::error::Error for Error {}
