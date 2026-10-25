use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

/// "ok" for a success, "failed with code N" for any other exit code, and
/// "killed by signal N" when a signal ended the program (it has no exit code then).
fn describe(status: ExitStatus) -> String {
    todo!("status.success(), status.code() (an Option) and status.signal() (also an Option)")
}

fn run(script: &str) -> ExitStatus {
    Command::new("sh")
        .args(["-c", script])
        .status()
        .expect("sh should start")
}

fn main() {
    assert_eq!(describe(run("exit 0")), "ok");
    assert_eq!(describe(run("exit 7")), "failed with code 7");
    assert_eq!(describe(run("exit 1")), "failed with code 1");
    assert_eq!(describe(run("kill -9 $$")), "killed by signal 9");
    assert_eq!(describe(run("kill -15 $$")), "killed by signal 15");
    assert_eq!(
        describe(Command::new("false").status().expect("false should start")),
        "failed with code 1"
    );

    println!("all checks passed");
}
