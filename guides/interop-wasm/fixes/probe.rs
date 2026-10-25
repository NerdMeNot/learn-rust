use std::io::ErrorKind;
use std::process::Command;
use std::thread;

fn main() {
    // Treat "this target cannot do that" as an answer, not a surprise.
    match Command::new("echo").arg("hi").output() {
        Ok(out) => print!("echo said: {}", String::from_utf8_lossy(&out.stdout)),
        Err(e) if e.kind() == ErrorKind::Unsupported => println!("no processes on this target"),
        Err(e) => println!("echo failed: {e}"),
    }

    let work = || 1 + 1;
    let answer = match thread::Builder::new().spawn(work) {
        Ok(handle) => handle.join().unwrap(),
        Err(_) => work(), // no threads here: do the work on the calling thread
    };
    println!("answer: {answer}");
}
