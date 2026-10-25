use std::{path::PathBuf, process::ExitCode};

use indexer::{index, report, walk};

#[tokio::main]
async fn main() -> ExitCode {
    let Some(root) = std::env::args().nth(1).map(PathBuf::from) else {
        eprintln!("usage: indexer <dir>");
        return ExitCode::from(2);
    };
    let files = match walk::list_files(&root).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("cannot read {}: {e}", root.display());
            return ExitCode::FAILURE;
        }
    };

    let mut results = report::Results::new();
    for path in files {
        let stats = index::index_file(path.clone()).await;
        results.insert(path, stats);
    }

    let (text, failed) = report::render(&results);
    print!("{text}");
    if failed > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
