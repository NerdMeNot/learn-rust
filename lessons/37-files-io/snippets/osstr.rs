use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir = std::env::temp_dir().join("lr-osstr");
    fs::create_dir_all(&dir)?;

    // a Linux file name is any bytes except '/' and NUL; this one is not UTF-8
    let name = OsStr::from_bytes(b"caf\xe9.txt");
    fs::write(dir.join(name), "hi")?;

    for entry in fs::read_dir(&dir)? {
        let name = entry?.file_name();
        println!("to_str: {:?}", name.to_str());
        println!("to_string_lossy: {}", name.to_string_lossy());
        println!("as_bytes: {:?}", name.as_bytes());
        println!("Debug: {:?}", name);
        println!("round trip works: {}", Path::new(&dir).join(&name).exists());
    }

    fs::remove_dir_all(&dir)
}
