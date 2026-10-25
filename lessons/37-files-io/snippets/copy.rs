use std::fs::{self, File};
use std::io;

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir();
    let (a, b) = (dir.join("lr-copy-a.txt"), dir.join("lr-copy-b.txt"));
    fs::write(&a, "some bytes\n")?;

    let n = fs::copy(&a, &b)?; // file to file, returns the byte count
    println!(
        "fs::copy: {n} bytes, copy reads {:?}",
        fs::read_to_string(&b)?
    );

    // io::copy moves bytes from any Read to any Write
    let mut sink = Vec::new();
    let n = io::copy(&mut File::open(&a)?, &mut sink)?;
    println!("io::copy into a Vec<u8>: {n} bytes");
    print!("io::copy to stdout: ");
    io::copy(&mut File::open(&a)?, &mut io::stdout().lock())?;

    fs::remove_file(&a)?;
    fs::remove_file(&b)
}
