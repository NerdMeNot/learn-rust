use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    if env::args().nth(1).as_deref() == Some("--child") {
        // exec replaces this process: it returns only when it fails
        let err = Command::new("echo").arg("replaced").exec();
        println!("exec returned: {err}");
        return;
    }
    let me = env::current_exe().expect("the path of this program");
    let out = Command::new(me).arg("--child").output().expect("child");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "replaced\n");
    assert!(out.status.success());
    println!("all checks passed");
}
