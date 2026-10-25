use std::io;
use std::process::{self, Command};
use std::thread;
use std::time::Duration;

/// Runs `script` with `sh -c` in the directory `dir`. The child inherits this process's
/// environment, also sees every `(name, value)` pair in `vars`, and reads `input` as its
/// standard input. Returns what the script printed on standard output and its exit code
/// (`None` if a signal ended it). Standard error is left alone. A directory that does not
/// exist is an `Err`; a script that exits non-zero is not.
fn run_script(
    script: &str,
    dir: &str,
    vars: &[(&str, &str)],
    input: &str,
) -> io::Result<(String, Option<i32>)> {
    todo!(
        "start sh -c in dir with the extra variables, give it a stdin pipe you write the input to, and make sure it sees the end of that input before you wait for it"
    )
}

fn main() {
    // A safety net for a wrong answer that waits for input that never ends. The checks
    // themselves use no clock.
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(10));
        eprintln!("stuck for 10 seconds: is the child's stdin closed after you wrote to it?");
        process::exit(1);
    });

    assert_eq!(
        run_script("echo hi", "/", &[], "").unwrap(),
        ("hi\n".to_string(), Some(0))
    );
    // the directory is the child's own
    assert_eq!(run_script("pwd", "/usr", &[], "").unwrap().0, "/usr\n");
    // variables reach the child
    let (out, _) = run_script(
        "echo \"$GREETING, $NAME\"",
        "/",
        &[("GREETING", "hello"), ("NAME", "world")],
        "",
    )
    .unwrap();
    assert_eq!(out, "hello, world\n");
    // nothing else is added or dropped: the child sees what a plain `sh` sees, plus the pairs
    let lines = |vars: &[(&str, &str)]| -> usize {
        run_script("env | wc -l", "/", vars, "")
            .unwrap()
            .0
            .trim()
            .parse()
            .unwrap()
    };
    let plain = Command::new("sh")
        .args(["-c", "env | wc -l"])
        .current_dir("/")
        .output()
        .unwrap();
    let plain: usize = String::from_utf8_lossy(&plain.stdout)
        .trim()
        .parse()
        .unwrap();
    assert_eq!(lines(&[]), plain);
    assert_eq!(lines(&[("RBC_X", "1"), ("RBC_Y", "2")]), plain + 2);
    // the input reaches the child's stdin, and the pipe is closed so `cat` can finish
    assert_eq!(
        run_script("cat", "/", &[], "one\ntwo\n").unwrap(),
        ("one\ntwo\n".to_string(), Some(0))
    );
    assert_eq!(
        run_script("tr a-z A-Z", "/", &[], "shout\n").unwrap().0,
        "SHOUT\n"
    );
    // a failing script is Ok with its code; a signal leaves no code
    assert_eq!(
        run_script("echo bye; exit 3", "/", &[], "").unwrap(),
        ("bye\n".to_string(), Some(3))
    );
    assert_eq!(
        run_script("kill -9 $$", "/", &[], "").unwrap(),
        (String::new(), None)
    );
    // a directory that does not exist is an error
    let err = run_script("true", "/no/such/directory", &[], "").unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);

    println!("all checks passed");
}
