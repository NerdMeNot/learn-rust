use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Replace `path` with `contents`: write a temporary file next to it, then rename it over.
fn save_atomic(path: &Path, contents: &str) -> io::Result<()> {
    let mut name = path.file_name().expect("a file path").to_owned();
    name.push(format!(".{}.tmp", std::process::id()));
    let tmp = path.with_file_name(name); // same directory, so the same file system

    // create_new fails if the name is taken, so we never touch a file that is not ours
    let mut f = OpenOptions::new().write(true).create_new(true).open(&tmp)?;

    // a closure that is called at once: `?` leaves the closure, not save_atomic,
    // so the clean-up below runs on any error
    let result = (|| {
        f.write_all(contents.as_bytes())?;
        f.sync_all()?; // ask the OS to put the bytes on the disk before the swap
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp); // do not leave the temporary file behind
    }
    result
}

fn names(dir: &Path) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        names.push(entry?.file_name().to_string_lossy().into_owned());
    }
    names.sort();
    Ok(names)
}

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("lr-atomic");
    fs::create_dir_all(&dir)?;
    let config = dir.join("config.txt");

    fs::write(&config, "version 1\n")?;
    save_atomic(&config, "version 2\n")?;
    println!("config: {:?}", fs::read_to_string(&config)?);
    println!("directory after the save: {:?}", names(&dir)?);

    // a save that fails: the target is a directory, so the rename is refused
    let blocked = dir.join("blocked");
    fs::create_dir(&blocked)?;
    let err = save_atomic(&blocked, "x").unwrap_err();
    println!("save over a directory: {:?}", err.kind());
    println!("directory after the failed save: {:?}", names(&dir)?);

    fs::remove_dir_all(&dir)
}
