use std::{path::PathBuf, rc::Rc};

use indexer::pipeline;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let (_tx, rx) = mpsc::channel::<PathBuf>(1);
    let label = Rc::new("worker");
    let outcome = pipeline::run(rx, 2, CancellationToken::new(), move |_path| {
        let label = label.clone();
        async move {
            tokio::task::yield_now().await;
            println!("{label}");
        }
    })
    .await;
    println!("{}", outcome.done.len());
}
