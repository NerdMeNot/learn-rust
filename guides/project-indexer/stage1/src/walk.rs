use std::{
    io,
    path::{Path, PathBuf},
};

/// Every file under `root`, in sorted order. The operating system lists a directory in
/// whatever order it likes, so sorting is what makes the result the same on every run.
pub async fn list_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let mut entries = tokio::fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if entry.file_type().await?.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn finds_every_file_in_the_sample_tree() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("sample");
        let found: Vec<String> = list_files(&root)
            .await
            .unwrap()
            .iter()
            .map(|p| p.strip_prefix(&root).unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(found, ["a.txt", "b.txt", "notes/deep/empty.txt", "notes/todo.txt"]);
    }

    #[tokio::test]
    async fn a_missing_directory_is_an_error() {
        let err = list_files(Path::new("no/such/dir")).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
