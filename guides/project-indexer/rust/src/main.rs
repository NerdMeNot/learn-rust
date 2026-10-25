use std::{path::PathBuf, process::ExitCode};

use indexer::{index, pipeline, report, walk};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

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

    // Ctrl-C cancels the whole pipeline instead of killing the process mid-write.
    let cancel = CancellationToken::new();
    let on_ctrl_c = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            on_ctrl_c.cancel();
        }
    });

    let (tx, rx) = mpsc::channel(8);
    let walker = tokio::spawn(walk::walk(root.clone(), tx, cancel.clone()));
    let outcome = pipeline::run(rx, jobs, cancel, index::index_file).await;
    if let Err(e) = walker.await.expect("the walker panicked") {
        eprintln!("cannot read {}: {e}", root.display());
        return ExitCode::FAILURE;
    }

    let (text, failed) = report::render(&outcome.done);
    print!("{text}");
    if outcome.cancelled {
        println!("cancelled: the table above is partial");
        return ExitCode::FAILURE;
    }
    if failed > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
