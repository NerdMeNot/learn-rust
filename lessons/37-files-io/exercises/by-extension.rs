use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Group the regular files directly inside `dir` by extension.
/// The key is the extension in lower case and without the dot (`Path::extension` already
/// leaves it off, where Python's `suffix` and C++'s `extension()` keep it), or "" for a file
/// without one; each list holds file names in sorted order. Directories and anything nested
/// deeper are left out. A name or extension that is not valid UTF-8 goes through
/// `to_string_lossy`. Linux only: macOS refuses to create a name that is not UTF-8.
fn group_by_extension(dir: &Path) -> io::Result<BTreeMap<String, Vec<String>>> {
    todo!("read_dir, skip non-files, take Path::extension and file_name from each entry")
}

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn main() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let dir = std::env::temp_dir().join(format!("lr-by-extension-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("sub.d")).unwrap();
    let scratch = Scratch(dir.clone());
    // created in no particular order: some file systems list newest first
    for name in [
        "m.txt",
        "b.txt",
        "q.txt",
        "a.TXT",
        "Makefile",
        "t.txt",
        ".hidden",
        "archive.tar.gz",
        "f.txt",
        "notes.md",
        "z.txt",
        "d.Txt",
        "k.txt",
    ] {
        fs::write(dir.join(name), name).unwrap();
    }
    fs::write(dir.join("sub.d/inner.txt"), "nested").unwrap();
    // the extension is not UTF-8: Linux file names are bytes
    fs::write(dir.join(OsStr::from_bytes(b"cafe.\xe9")), "not utf-8").unwrap();

    let got = group_by_extension(&dir).unwrap();
    let show = |key: &str| got.get(key).cloned().unwrap_or_default();
    // sorting is checked by the order read_dir happens to return, which depends on the
    // file system: a solution that forgets the sort may pass where that order is sorted
    assert_eq!(
        show("txt"),
        [
            "a.TXT", "b.txt", "d.Txt", "f.txt", "k.txt", "m.txt", "q.txt", "t.txt", "z.txt"
        ],
        "txt: lower-cased key, sorted names, nothing nested"
    );
    assert_eq!(
        show("\u{fffd}"),
        ["cafe.\u{fffd}"],
        "an extension that is not UTF-8 still gets a key"
    );
    assert_eq!(
        show("gz"),
        ["archive.tar.gz"],
        "only the last extension counts"
    );
    assert_eq!(show("md"), ["notes.md"]);
    assert_eq!(
        show(""),
        [".hidden", "Makefile"],
        "a leading dot is not an extension"
    );
    assert_eq!(got.len(), 5, "keys: {:?}", got.keys().collect::<Vec<_>>());

    let err = group_by_extension(&dir.join("nope")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);

    drop(scratch);
    println!("all checks passed");
}
