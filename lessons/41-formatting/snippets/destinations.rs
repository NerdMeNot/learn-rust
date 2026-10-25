use std::fmt::Write as _; // for write! into a String
use std::io::Write as _; // for write! into bytes, files, stdout

fn main() {
    let mut text = String::new();
    write!(text, "{}-{}", 1, 2).unwrap();
    writeln!(text, "!").unwrap();
    println!("String: {:?}", text);

    let mut bytes: Vec<u8> = Vec::new();
    write!(bytes, "{}-{}", 1, 2).unwrap();
    println!("Vec<u8>: {:?}", bytes);

    let mut out = std::io::stdout().lock();
    writeln!(out, "stdout: through write!").unwrap();
    writeln!(out, "also stdout").unwrap();
    drop(out); // release the lock so that other threads can print

    eprintln!("stderr: eprintln!");
    println!("done");
}
