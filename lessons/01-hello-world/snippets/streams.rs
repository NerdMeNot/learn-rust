use std::process::Command;

// The program under test: it prints on both streams.
fn child() {
    print!("loading... ");
    println!("done");
    eprintln!("warning: cache is cold");
    let doubled = dbg!(21 * 2);
    println!("doubled: {doubled}");
}

// This same program runs itself with the argument `child` and keeps the two streams
// apart, which a terminal does not: it shows them mixed.
fn main() {
    if std::env::args().nth(1).as_deref() == Some("child") {
        return child();
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .arg("child")
        .output()
        .unwrap();
    println!("exit code: {:?}", out.status.code());
    println!("--- stdout ---\n{}", String::from_utf8_lossy(&out.stdout));
    println!("--- stderr ---\n{}", String::from_utf8_lossy(&out.stderr));
}
