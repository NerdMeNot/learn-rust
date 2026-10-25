use std::process::Command;

fn main() {
    // a name without a slash is looked up in PATH, like execvp does
    let out = Command::new("echo").arg("found").output().expect("echo");
    println!("echo: {:?}", String::from_utf8_lossy(&out.stdout));

    // PATH set for the child alone: now the lookup finds nothing
    let err = Command::new("echo")
        .env("PATH", "/nonexistent")
        .output()
        .unwrap_err();
    println!("PATH=/nonexistent: {:?}", err.kind());

    // a name with a slash is a path: nothing is searched
    let out = Command::new("/bin/echo")
        .arg("by path")
        .env("PATH", "/nonexistent")
        .output()
        .expect("/bin/echo");
    println!("/bin/echo: {:?}", String::from_utf8_lossy(&out.stdout));
}
