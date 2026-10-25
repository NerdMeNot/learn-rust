use std::fs;
use std::io;
use std::path::PathBuf;

/// A scratch directory that deletes itself when it goes out of scope.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> io::Result<TempDir> {
        // the process id keeps two runs that overlap in time from sharing a directory
        let path = std::env::temp_dir().join(format!("lr-{tag}-{}", std::process::id()));
        fs::create_dir(&path)?; // fails with AlreadyExists if an old run left it behind
        Ok(TempDir(path))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0); // nothing useful to do if this fails
    }
}

/// Returns early with `?`, and the guard `tmp` is still dropped.
fn work(seen: &mut Option<PathBuf>) -> io::Result<()> {
    let tmp = TempDir::new("demo")?;
    *seen = Some(tmp.0.clone());
    fs::write(tmp.0.join("a.txt"), "data")?;
    println!("inside work: exists {}", tmp.0.exists());
    let _ = fs::read_to_string(tmp.0.join("missing.txt"))?; // fails: we leave early
    Ok(())
}

fn main() {
    let mut seen = None;
    println!("work: {:?}", work(&mut seen).map_err(|e| e.kind()));
    println!("after work: exists {}", seen.unwrap().exists());
}
