use std::io::Read;
use std::{env, fs, io};

fn main() {
    // A WASI module is a program: it has main, arguments, environment, stdin and stdout.
    println!("args: {:?}", env::args().skip(1).collect::<Vec<_>>());
    println!("GREETING = {:?}", env::var("GREETING"));

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("stdin had {} bytes", input.len());

    // Files: only the directories the host hands over ("preopens") exist for the module.
    println!("write sandbox/note.txt: {:?}", fs::write("sandbox/note.txt", input.to_uppercase()));
    println!("read  sandbox/note.txt: {:?}", fs::read_to_string("sandbox/note.txt"));
    println!("read  /etc/hostname:    {:?}", fs::read_to_string("/etc/hostname").map_err(|e| e.kind()));
}
