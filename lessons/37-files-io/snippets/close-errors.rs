use std::fs::File;
use std::io::{self, BufWriter, Write};

// /dev/full accepts open() and refuses every byte, like a full disk (Linux)
fn forget_to_flush() -> io::Result<()> {
    let mut w = BufWriter::new(File::create("/dev/full")?);
    println!(
        "write_all: {:?}",
        w.write_all(b"data").map_err(|e| e.kind())
    );
    Ok(()) // w is dropped here; its flush fails and nobody is told
}

fn flush_and_check() -> io::Result<()> {
    let mut w = BufWriter::new(File::create("/dev/full")?);
    w.write_all(b"data")?;
    w.flush()
}

fn main() {
    println!(
        "without flush: {:?}",
        forget_to_flush().map_err(|e| e.kind())
    );
    println!(
        "with flush()?: {:?}",
        flush_and_check().map_err(|e| e.kind())
    );
}
