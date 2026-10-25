use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("--child") {
        // args[0] is the program and args[1] is "--child"; the rest came from the parent
        println!("args.len(): {}", args.len());
        for (i, arg) in args.iter().enumerate().skip(2) {
            println!("args[{i}]: {arg:?}");
        }
        println!("nth(10): {:?}", env::args().nth(10));
        return;
    }

    // the part that starts a copy of this program with arguments
    let me = env::current_exe().expect("the path of this program");
    let out = Command::new(me)
        .args(["--child", "hello world", "*", "--flag=1", "naïve"])
        .output()
        .expect("the child should start");
    print!("{}", String::from_utf8_lossy(&out.stdout));
}
