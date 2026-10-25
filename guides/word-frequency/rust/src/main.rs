mod args;
mod count;
mod error;
mod input;
mod report;
mod words;

use std::env;
use std::process::ExitCode;

use error::CliError;

fn run() -> Result<(), CliError> {
    let config = args::parse_args(env::args().skip(1))?;
    let text = input::read_input(config.path.as_deref())?;
    let counts = count::count_words(&text);
    let total: usize = counts.values().sum();
    let distinct = counts.len();
    let top = count::top_n(counts, config.top);
    report::print(total, distinct, &top);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("wordcount: {err}");
            if matches!(err, CliError::Usage(_)) {
                eprintln!("usage: wordcount [FILE] [--top N]");
            }
            ExitCode::from(err.exit_code())
        }
    }
}
