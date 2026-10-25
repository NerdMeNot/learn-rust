use crate::error::CliError;

pub const DEFAULT_TOP: usize = 10;

pub struct Config {
    /// `None` means "read standard input".
    pub path: Option<String>,
    pub top: usize,
}

fn usage(message: impl Into<String>) -> CliError {
    CliError::Usage(message.into())
}

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, CliError> {
    let mut path = None;
    let mut top = DEFAULT_TOP;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--top" => {
                let value = args.next().ok_or_else(|| usage("--top needs a number"))?;
                top = match value.parse::<usize>() {
                    Ok(n) if n >= 1 => n,
                    Ok(_) => return Err(usage("--top must be at least 1")),
                    Err(e) => return Err(usage(format!("invalid value {value:?} for --top: {e}"))),
                };
            }
            flag if flag.starts_with('-') && flag != "-" => {
                return Err(usage(format!("unknown option {flag}")));
            }
            _ if path.is_some() => return Err(usage("only one input file is allowed")),
            "-" => {}
            _ => path = Some(arg),
        }
    }
    Ok(Config { path, top })
}
