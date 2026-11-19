//! Runs `summarize` on real files in a temporary directory.

use std::error::Error;
use std::path::PathBuf;

use logstat::{LogError, Stats, summarize};

/// A directory under the system's temp dir, removed when the test ends.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> std::io::Result<Self> {
        let dir = std::env::temp_dir().join(format!("logstat-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir)?;
        Ok(Self(dir))
    }

    /// Writes a file and returns its path as the program would get it.
    fn file(&self, name: &str, text: &str) -> std::io::Result<String> {
        let path = self.0.join(name);
        std::fs::write(&path, text)?;
        Ok(path.display().to_string())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_dir_all(&self.0) {
            eprintln!("could not remove {}: {err}", self.0.display());
        }
    }
}

#[tokio::test]
async fn adds_up_several_files() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new("adds-up")?;
    let a = dir.file("a.log", "t INFO GET / ms=5\nt ERROR POST / ms=900\n")?;
    let b = dir.file("b.log", "t WARN slow ms=300\n\nt INFO GET /x\n")?;

    let stats = summarize(vec![a, b]).await?;
    assert_eq!(stats, Stats { error: 1, warn: 1, info: 2, slowest_ms: 900 });
    Ok(())
}

#[tokio::test]
async fn a_missing_file_is_an_error_that_names_it() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new("missing")?;
    let a = dir.file("a.log", "t INFO GET / ms=5\n")?;
    let missing = dir.0.join("missing.log").display().to_string();

    match summarize(vec![a, missing.clone()]).await {
        Err(LogError::Read { path, .. }) => assert_eq!(path, missing),
        other => panic!("expected a read error, got {other:?}"),
    }
    Ok(())
}

#[tokio::test]
async fn a_bad_line_names_its_file_and_line() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new("bad-line")?;
    let a = dir.file("a.log", "t INFO GET / ms=5\nt INFO GET / ms=5s\n")?;

    let err = summarize(vec![a.clone()]).await.err().ok_or("expected an error")?;
    assert_eq!(
        err.to_string(),
        format!("{a}, line 2: duration \"5s\" is not a whole number of milliseconds")
    );
    Ok(())
}
