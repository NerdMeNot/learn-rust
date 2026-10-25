use std::process::Command;

fn main() {
    // output() waits, captures stdout and stderr, and gives the exit status
    let out = Command::new("echo")
        .arg("hello from echo")
        .output()
        .expect("echo should run");
    println!("stdout: {:?}", String::from_utf8_lossy(&out.stdout));
    println!("success: {}", out.status.success());

    // arguments go to the program as they are: no shell splits, expands or runs them
    let out = Command::new("echo")
        .args(["a  b", "$HOME", "*", "one; echo two"])
        .output()
        .expect("echo should run");
    println!("literal: {:?}", String::from_utf8_lossy(&out.stdout));

    // the whole string is taken as a file name
    let err = Command::new("echo hello").output().unwrap_err();
    println!("\"echo hello\": {:?}, {err}", err.kind());

    // a program that runs and fails is Ok(status), not an Err
    let status = Command::new("false").status().expect("false should run");
    println!(
        "false: success {}, code {:?}",
        status.success(),
        status.code()
    );

    // stdout and stderr are separate byte buffers
    let out = Command::new("sh")
        .args(["-c", "echo on-stdout; echo on-stderr >&2; exit 4"])
        .output()
        .expect("sh should run");
    println!("stdout {:?}", String::from_utf8_lossy(&out.stdout));
    println!("stderr {:?}", String::from_utf8_lossy(&out.stderr));
    println!("code {:?}", out.status.code());
}
