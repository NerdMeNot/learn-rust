use std::{path::PathBuf, process::ExitCode};

use indexer::{index, pipeline, report, walk};
use tokio::sync::mpsc;

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

    let (tx, rx) = mpsc::channel(8);
    let walker = tokio::spawn(walk::walk(root.clone(), tx));
    let results = pipeline::run(rx, jobs, index::index_file).await;
    if let Err(e) = walker.await.expect("the walker panicked") {
        eprintln!("cannot read {}: {e}", root.display());
        return ExitCode::FAILURE;
    }

    let (text, failed) = report::render(&results);
    print!("{text}");
    if failed > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
