use std::{path::PathBuf, process::ExitCode};

use indexer::{index, pipeline, report, walk};

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(root) = args.next().map(PathBuf::from) else {
        eprintln!("usage: indexer <dir> [jobs]");
        return ExitCode::from(2);
    };
    let jobs = match args.next() {
        None => 4,
        Some(text) => match text.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("jobs must be a positive number, not {text:?}");
                return ExitCode::from(2);
            }
        },
    };
    let files = match walk::list_files(&root).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("cannot read {}: {e}", root.display());
            return ExitCode::FAILURE;
        }
    };

    let results = pipeline::run(files, jobs, index::index_file).await;

    let (text, failed) = report::render(&results);
    print!("{text}");
    if failed > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
