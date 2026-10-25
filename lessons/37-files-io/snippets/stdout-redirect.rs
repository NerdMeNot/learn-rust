use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::process::Command;

/// How many write system calls this process has made so far (Linux: the `syscw` line).
fn write_calls() -> u64 {
    let io = fs::read_to_string("/proc/self/io").unwrap();
    let line = io.lines().find(|l| l.starts_with("syscw:")).unwrap();
    line["syscw:".len()..].trim().parse().unwrap()
}

fn main() -> io::Result<()> {
    let mode = std::env::args().nth(1);
    match mode.as_deref() {
        Some("plain") => {
            let before = write_calls();
            for i in 0..1000 {
                println!("line {i}");
            }
            eprintln!("  1000 println!: {} write calls", write_calls() - before);
        }
        Some("buffered") => {
            let before = write_calls();
            let mut out = BufWriter::new(io::stdout().lock());
            for i in 0..1000 {
                writeln!(out, "line {i}")?;
            }
            out.flush()?;
            eprintln!(
                "  1000 writeln! into a BufWriter: {} write calls",
                write_calls() - before
            );
        }
        _ => {
            // run this program again, twice, with standard output going to a file
            let path = std::env::temp_dir().join("lr-stdout-redirect.txt");
            for mode in ["plain", "buffered"] {
                println!("child {mode}, stdout redirected to a file:");
                Command::new(std::env::current_exe()?)
                    .arg(mode)
                    .stdout(File::create(&path)?)
                    .status()?;
            }
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}
