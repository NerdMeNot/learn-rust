use std::process::Command;

fn main() {
    // output() gives the child a stdin that is already at end of input, so `cat` finishes
    let out = Command::new("cat").output().expect("cat should run");
    assert!(out.status.success());
    assert!(out.stdout.is_empty());

    // output() captures stdout and stderr; status() does not capture
    let out = Command::new("sh")
        .args(["-c", "echo a; echo b >&2"])
        .output()
        .expect("sh should run");
    assert_eq!(out.stdout, b"a\n");
    assert_eq!(out.stderr, b"b\n");

    // a missing program is an Err; a failing program is Ok
    assert!(Command::new("no-such-program-here").status().is_err());
    assert!(!Command::new("false").status().unwrap().success());

    // Duration has no Display; Debug is the printable form
    assert_eq!(
        format!("{:?}", std::time::Duration::from_millis(1500)),
        "1.5s"
    );
    println!("all checks passed");
}
