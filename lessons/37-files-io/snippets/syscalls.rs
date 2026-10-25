use std::fs::{self, File};
use std::io::{self, BufWriter, Write};

/// How many write system calls this process has made so far (Linux: the `syscw` line).
fn write_calls() -> u64 {
    let io = fs::read_to_string("/proc/self/io").unwrap();
    let line = io.lines().find(|l| l.starts_with("syscw:")).unwrap();
    line["syscw:".len()..].trim().parse().unwrap()
}

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("lr-syscalls.txt");

    let mut file = File::create(&path)?;
    let before = write_calls();
    for i in 0..1000 {
        writeln!(file, "line {i}")?;
    }
    println!("unbuffered File: {} write calls", write_calls() - before);

    // a File has no buffer, so there is nothing for flush to do
    let before = write_calls();
    for _ in 0..1000 {
        file.flush()?;
    }
    println!(
        "1000 File::flush calls: {} write calls",
        write_calls() - before
    );

    let mut w = BufWriter::new(File::create(&path)?);
    let before = write_calls();
    for i in 0..1000 {
        writeln!(w, "line {i}")?;
    }
    w.flush()?;
    println!("BufWriter<File>: {} write calls", write_calls() - before);

    println!("bytes in the file: {}", fs::metadata(&path)?.len());
    fs::remove_file(&path)
}
