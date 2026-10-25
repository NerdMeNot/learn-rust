use std::fs;
use std::io::{self, Read};

use crate::error::CliError;

pub fn read_input(path: Option<&str>) -> Result<String, CliError> {
    match path {
        Some(path) => fs::read_to_string(path).map_err(|cause| CliError::Read {
            what: path.to_string(),
            cause,
        }),
        None => {
            let mut text = String::new();
            io::stdin()
                .read_to_string(&mut text)
                .map_err(|cause| CliError::Read {
                    what: "standard input".to_string(),
                    cause,
                })?;
            Ok(text)
        }
    }
}
