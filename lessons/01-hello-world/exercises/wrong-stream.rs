use std::process::Command;

/// The program under test. Its answer belongs on standard output; its warning is a
/// message about the run, not part of the answer, so it belongs on standard error.
fn child() {
    println!("warning: no config file, using defaults");
    let answer = 6 * 7;
    println!("result: {answer}");
}

// Runs `child` as a separate process so that the two streams can be told apart.
fn main() {
    if std::env::args().nth(1).as_deref() == Some("child") {
        return child();
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .arg("child")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout), "result: 42\n");
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "warning: no config file, using defaults\n"
    );

    println!("all checks passed");
}
