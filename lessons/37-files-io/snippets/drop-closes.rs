use std::fs::{self, File};
use std::io;

/// How many file descriptors this process has open (Linux: one entry in /proc/self/fd each).
fn open_fds() -> usize {
    fs::read_dir("/proc/self/fd").unwrap().count()
}

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("lr-drop-closes.txt");
    fs::write(&path, "x")?;
    let base = open_fds();

    {
        let _f = File::open(&path)?;
        println!("inside the block: {} more open", open_fds() - base);
    }
    println!("after the block: {} more open", open_fds() - base);

    let f = File::open(&path)?;
    println!("opened again: {} more open", open_fds() - base);
    drop(f);
    println!("after drop(f): {} more open", open_fds() - base);

    let f = File::open(&path)?;
    std::mem::forget(f); // never dropped, so never closed
    println!("after mem::forget: {} more open", open_fds() - base);

    fs::remove_file(&path)
}
