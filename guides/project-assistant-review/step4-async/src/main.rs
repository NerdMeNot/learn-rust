use std::error::Error;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    let files = paths.len();

    match logstat::summarize(paths).await {
        Ok(stats) => {
            println!("files   {files}");
            println!("ERROR   {}", stats.error);
            println!("WARN    {}", stats.warn);
            println!("INFO    {}", stats.info);
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
