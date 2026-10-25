use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("lr-open-options.txt");

    // create_new: create it, and fail if it already exists (one step, no check first)
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    f.write_all(b"first\n")?;
    let again = OpenOptions::new().write(true).create_new(true).open(&path);
    println!(
        "create_new twice: {:?}",
        again.map(|_| ()).map_err(|e| e.kind())
    );

    // append: every write goes to the end
    let mut f = OpenOptions::new().append(true).open(&path)?;
    f.write_all(b"second\n")?;
    println!("after append: {:?}", fs::read_to_string(&path)?);

    // append without create does not make a missing file
    let missing = OpenOptions::new()
        .append(true)
        .open(path.with_extension("none"));
    println!(
        "append to a missing file: {:?}",
        missing.map(|_| ()).map_err(|e| e.kind())
    );

    // File::create is write + create + truncate
    File::create(&path)?;
    println!("after File::create: {} bytes", fs::metadata(&path)?.len());

    // File::open is read-only, so a write through it fails
    let err = File::open(&path)?.write_all(b"x").unwrap_err();
    println!("write on a read-only handle: {err}");

    fs::remove_file(&path)?;

    // a mode argument, like os.WriteFile's 0o644 (Unix only; the umask can remove bits)
    let private = path.with_extension("private");
    let f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&private)?;
    println!(
        "mode(0o600): {:o}",
        f.metadata()?.permissions().mode() & 0o777
    );
    fs::set_permissions(&private, fs::Permissions::from_mode(0o640))?;
    println!(
        "after set_permissions(0o640): {:o}",
        fs::metadata(&private)?.permissions().mode() & 0o777
    );
    fs::remove_file(&private)
}
