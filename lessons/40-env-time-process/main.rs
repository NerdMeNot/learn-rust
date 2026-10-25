use std::env;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    // args: the first one is the program itself
    let args: Vec<String> = env::args().collect();
    println!("{} arg(s) after the program name", args.len() - 1);

    // env vars return Result, because the variable might not be set
    let port = env::var("RBC_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("port {port}");
    match env::var("RBC_MISSING") {
        Ok(v) => println!("RBC_MISSING={v}"),
        Err(e) => println!("RBC_MISSING: {e}"),
    }

    // Instant measures, Duration is an amount of time
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(20));
    let took = start.elapsed();
    println!("slept at least 20ms: {}", took >= Duration::from_millis(20));

    // run another program and capture what it prints
    let out = Command::new("echo")
        .arg("hello from echo")
        .output()
        .expect("echo should run");
    print!("{}", String::from_utf8_lossy(&out.stdout));
    println!("exit ok: {}", out.status.success());
}
