use std::io::Write;
use std::process::Command;

// The program under test prints, then ends: with `abort()`, which skips normal cleanup,
// except in the `normal-exit` mode, which simply returns.
fn child(mode: &str) {
    match mode {
        "line" => println!("a line, with a newline"),
        "partial" => print!("a partial line"),
        "normal-exit" => return print!("a partial line, normal exit"),
        _ => {
            print!("a partial line, flushed");
            std::io::stdout().flush().unwrap();
        }
    }
    std::process::abort();
}

fn main() {
    if let Some(mode) = std::env::args().nth(1) {
        return child(&mode);
    }
    for mode in ["line", "partial", "flushed", "normal-exit"] {
        let out = Command::new(std::env::current_exe().unwrap())
            .arg(mode)
            .output()
            .unwrap();
        println!(
            "{mode}: stdout is {:?}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}
