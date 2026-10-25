use std::env;
use std::io::{BufWriter, Write, stdout};
use std::process::{self, Command};

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  dropped {}", self.0);
    }
}

fn run(how: &str) {
    let _guard = Guard("the guard");
    println!("  running, how = {how}");
    if how == "exit" {
        process::exit(0);
    }
}

fn main() {
    if let Some(how) = env::args().nth(1) {
        if how == "exit-buffered" {
            println!("  running, how = {how}");
            // a BufWriter hands its bytes over when it is flushed or dropped
            let mut out = BufWriter::new(stdout());
            writeln!(out, "  written to a BufWriter").unwrap();
            process::exit(0);
        }
        run(&how);
        return;
    }
    let me = env::current_exe().expect("the path of this program");
    for how in ["return", "exit", "exit-buffered"] {
        println!("{how}:");
        let out = Command::new(&me).arg(how).output().expect("child");
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
}
