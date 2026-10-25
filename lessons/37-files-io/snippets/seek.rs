use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("lr-seek.txt");
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    f.write_all(b"0123456789")?;

    f.seek(SeekFrom::Start(3))?;
    let mut four = [0u8; 4];
    f.read_exact(&mut four)?;
    println!("seek to 3, read 4: {:?}", String::from_utf8_lossy(&four));
    println!("position now: {}", f.stream_position()?);

    f.seek(SeekFrom::End(-2))?; // two bytes before the end
    f.write_all(b"XY")?; // overwrites them
    f.rewind()?;
    let mut all = String::new();
    f.read_to_string(&mut all)?;
    println!("after overwriting the last two bytes: {all:?}");

    // a Cursor is the same three traits over memory: no disk, so a test can use it
    let mut mem = Cursor::new(Vec::new());
    mem.write_all(b"hello world")?;
    mem.set_position(6);
    let mut rest = String::new();
    mem.read_to_string(&mut rest)?;
    println!("cursor from byte 6: {rest:?}");

    fs::remove_file(&path)
}
