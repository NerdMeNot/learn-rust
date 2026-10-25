use std::env;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::process::Command;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("os") => {
            let arg = env::args_os().nth(2).expect("one argument");
            println!("to_str(): {:?}", arg.to_str());
            println!("to_string_lossy(): {}", arg.to_string_lossy());
            println!("var: {:?}", env::var("RBC_NAME"));
            println!("var_os is some: {}", env::var_os("RBC_NAME").is_some());
        }
        Some("args") => println!("{} arguments", env::args().count()),
        Some("vars") => println!("{} variables", env::vars().count()),
        _ => {
            // 0xE9 is "é" in Latin-1, but a lone 0xE9 is not valid UTF-8
            let bad = OsStr::from_bytes(b"caf\xe9");
            let me = env::current_exe().expect("the path of this program");
            for mode in ["os", "args", "vars"] {
                let out = Command::new(&me)
                    .arg(mode)
                    .arg(bad)
                    .env("RBC_NAME", bad)
                    .output()
                    .expect("the child should start");
                println!("{mode}: exit code {:?}", out.status.code());
                print!("{}", String::from_utf8_lossy(&out.stdout));
                let err = String::from_utf8_lossy(&out.stderr);
                if let Some(line) = err.lines().find(|l| l.contains("Err")) {
                    println!("stderr: {line}");
                }
            }
        }
    }
}
