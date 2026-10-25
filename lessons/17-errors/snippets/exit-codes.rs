use std::error::Error;
use std::process::{Command, exit};

// This program starts copies of itself and prints how each one ended.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("ok") => println!("fine"),
        Some("panic") => panic!("a bug"),
        Some("exit") => exit(2),
        Some("return-err") => {
            let n: u32 = "x".parse()?;
            println!("unreachable {n}");
        }
        _ => {
            // No argument: this is the parent. Start one child per mode.
            for mode in ["ok", "panic", "exit", "return-err"] {
                let me = std::env::current_exe()?;
                let status = Command::new(me).arg(mode).output()?.status;
                println!("{mode}: exit code {:?}", status.code());
            }
        }
    }
    Ok(())
}
