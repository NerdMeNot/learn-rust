use std::fs;
use std::io::{self, ErrorKind};

fn load(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    let err = load("/no/such/file.txt").unwrap_err();
    println!("display: {err}");
    println!("kind: {:?}", err.kind());
    println!("raw_os_error: {:?}", err.raw_os_error());

    // ErrorKind is #[non_exhaustive], so a match needs a catch-all arm
    for path in ["/no/such/file.txt", "/"] {
        let what = match load(path) {
            Ok(_) => "read it".to_string(),
            Err(e) if e.kind() == ErrorKind::NotFound => "does not exist".to_string(),
            Err(e) => format!("failed: {:?}", e.kind()),
        };
        println!("{path}: {what}");
    }

    // the message does not say which path failed; add that where you know it
    let with_path = load("/no/such/file.txt").map_err(|e| format!("/no/such/file.txt: {e}"));
    println!("with context: {with_path:?}");
}
