use std::error::Error;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();

    match logstat::summarize(&paths).await {
        Ok(stats) => {
            println!("files   {}", paths.len());
            for level in ["ERROR", "WARN", "INFO"] {
                let n = stats.counts.get(level).copied().unwrap_or(0);
                println!("{level:<7} {n}");
            }
            println!("slowest {} ms", stats.slowest_ms);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            let mut source = err.source();
            while let Some(cause) = source {
                eprintln!("  caused by: {cause}");
                source = cause.source();
            }
            ExitCode::FAILURE
        }
    }
}
