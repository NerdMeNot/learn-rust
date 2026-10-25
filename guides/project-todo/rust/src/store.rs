//! Reading the list from a JSON file and writing it back.
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::List;

/// A file that does not exist yet is an empty list; any other problem is an error.
pub fn load(path: &Path) -> Result<List> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(List::default()),
        Err(e) => return Err(e).with_context(|| format!("could not read {}", path.display())),
    };
    serde_json::from_str(&text)
        .with_context(|| format!("{} is not a valid to-do file", path.display()))
}

/// Write to `<path>.tmp`, then rename it over `path`.
pub fn save(list: &List, path: &Path) -> Result<()> {
    let mut tmp: OsString = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let json = serde_json::to_string_pretty(list)?;
    fs::write(&tmp, json + "\n").with_context(|| format!("could not write {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("could not replace {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A folder under the system temp dir, removed again when dropped.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("todo-{}-{name}", std::process::id()));
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }
        fn file(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_saved_list_loads_back_equal() {
        let dir = TempDir::new("round-trip");
        let path = dir.file("todo.json");
        let mut list = List::default();
        list.add("write the page", 1).unwrap();
        list.add("run the tests", 2).unwrap();
        list.complete(1).unwrap();
        save(&list, &path).unwrap();
        assert_eq!(load(&path).unwrap(), list);
        assert!(!dir.file("todo.json.tmp").exists());
    }

    #[test]
    fn a_missing_file_is_an_empty_list() {
        let dir = TempDir::new("missing");
        assert_eq!(load(&dir.file("nope.json")).unwrap(), List::default());
    }

    #[test]
    fn garbage_is_an_error_that_names_the_file() {
        let dir = TempDir::new("garbage");
        let path = dir.file("bad.json");
        fs::write(&path, "{oops").unwrap();
        let message = format!("{:#}", load(&path).unwrap_err());
        assert!(
            message.contains("bad.json is not a valid to-do file"),
            "{message}"
        );
    }
}
