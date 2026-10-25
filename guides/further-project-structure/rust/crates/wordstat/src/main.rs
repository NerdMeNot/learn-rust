//! Prints each word in its arguments with its count, most frequent first.

use std::process::ExitCode;

fn main() -> ExitCode {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    match wordstat::count_words(&text) {
        Ok(stats) => {
            for (word, n) in stats.most_common() {
                println!("{n:>3} {word}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("wordstat: {e}");
            ExitCode::FAILURE
        }
    }
}
