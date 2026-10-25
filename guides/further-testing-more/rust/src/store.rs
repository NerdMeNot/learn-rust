//! Notes kept as one file each, inside a folder the caller chooses.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn save_note(dir: &Path, name: &str, text: &str) -> io::Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let path = dir.join(format!("{name}.txt"));
    fs::write(&path, text)?;
    Ok(path)
}

pub fn load_note(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Names of the notes in `dir`, sorted.
pub fn note_names(dir: &Path) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(stem) = path.file_stem() else {
            continue;
        };
        if path.extension().is_some_and(|e| e == "txt") {
            names.push(stem.to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}
