use std::fs;
use std::io;
use std::path::Path;

/// std has no recursive walk: `read_dir` is one level, so recurse yourself
/// (the `walkdir` crate is the usual shortcut).
fn walk(dir: &Path, prefix: &str, found: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        if entry.file_type()?.is_dir() {
            walk(&entry.path(), &format!("{name}/"), found)?; // file_type does not follow symlinks
        }
        found.push(name);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = std::env::temp_dir().join("lr-dirs");
    fs::create_dir_all(root.join("a/b"))?; // every missing level; fine if it exists
    fs::write(root.join("top.txt"), "333")?;
    fs::write(root.join("a/one.txt"), "1")?;

    // read_dir yields Result items, in an order the docs do not specify: sort
    let mut names = Vec::new();
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let kind = if entry.file_type()?.is_dir() {
            "dir"
        } else {
            "file"
        };
        names.push(format!("{} ({kind})", entry.file_name().to_string_lossy()));
    }
    names.sort();
    println!("entries: {names:?}");

    let mut all = Vec::new();
    walk(&root, "", &mut all)?;
    all.sort();
    println!("walk: {all:?}");

    let meta = fs::metadata(root.join("top.txt"))?;
    println!(
        "top.txt: len {}, is_file {}, is_dir {}",
        meta.len(),
        meta.is_file(),
        meta.is_dir()
    );

    let err = fs::remove_dir(root.join("a")).unwrap_err();
    println!("remove_dir on a non-empty directory: {:?}", err.kind());

    fs::remove_dir_all(&root)?;
    println!("root exists afterwards: {:?}", root.try_exists());
    Ok(())
}
