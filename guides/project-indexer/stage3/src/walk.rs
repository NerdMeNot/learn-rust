use std::{io, path::PathBuf};

use tokio::sync::mpsc;

/// Sends every file under `root` into `tx`, one at a time, as it finds them. When the
/// channel is full, `send` waits, so the walk never gets far ahead of the workers.
pub async fn walk(root: PathBuf, tx: mpsc::Sender<PathBuf>) -> io::Result<()> {
    let mut dirs = vec![root];
    while let Some(dir) = dirs.pop() {
        let mut entries = tokio::fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if entry.file_type().await?.is_dir() {
                dirs.push(path);
            } else if tx.send(path).await.is_err() {
                return Ok(()); // the receiver is gone: nobody wants more paths
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[tokio::test]
    async fn finds_every_file_in_the_sample_tree() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("sample");
        let (tx, mut rx) = mpsc::channel(16);
        walk(root.clone(), tx).await.unwrap();
        let mut found = Vec::new();
        while let Some(path) = rx.recv().await {
            found.push(path.strip_prefix(&root).unwrap().to_string_lossy().into_owned());
        }
        found.sort(); // the walk's order is the operating system's, so sort before comparing
        assert_eq!(found, ["a.txt", "b.txt", "notes/deep/empty.txt", "notes/todo.txt"]);
    }

    #[tokio::test]
    async fn a_missing_directory_is_an_error() {
        let (tx, _rx) = mpsc::channel(1);
        let err = walk("no/such/dir".into(), tx).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
