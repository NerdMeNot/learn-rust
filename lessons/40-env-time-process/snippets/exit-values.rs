use std::env;
use std::process::{self, Command, ExitCode};

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        Some("from7") => ExitCode::from(7),
        Some("exit256") => process::exit(256),
        Some("exit300") => process::exit(300),
        Some("exit-1") => process::exit(-1),
        _ => {
            // no argument: start this same program once per case and report the exit code
            let me = env::current_exe().expect("the path of this program");
            for mode in ["from7", "exit256", "exit300", "exit-1"] {
                let status = Command::new(&me).arg(mode).status().expect("the child");
                println!("{mode:>7} -> {:?}", status.code());
            }
            ExitCode::SUCCESS
        }
    }
}
