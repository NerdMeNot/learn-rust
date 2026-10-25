use std::process::Command;
use std::thread;

fn main() {
    let echo = Command::new("echo").arg("hi").output();
    println!("run echo: {:?}", echo.map(|out| out.status.success()));

    let worker = thread::Builder::new().spawn(|| 1 + 1);
    println!("spawn thread: {:?}", worker.map(|handle| handle.join().unwrap()));
}
