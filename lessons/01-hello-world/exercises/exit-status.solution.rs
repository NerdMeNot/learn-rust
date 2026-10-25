use std::process::{Command, ExitCode};

/// The work, already written: halves a number given as text.
fn run(arg: Option<String>) -> Result<u32, String> {
    match arg {
        None => Err(String::from("missing number")),
        Some(text) => match text.parse::<u32>() {
            Ok(n) => Ok(n / 2),
            Err(_) => Err(format!("not a number: {text}")),
        },
    }
}

/// Turns the outcome into what the process reports. `Ok(n)`: print `n` on standard
/// output and succeed. `Err(message)`: print the message on standard error and exit with
/// code 2, a usage error. (Returning the `Err` from `main` would give 1.)
fn finish(outcome: Result<u32, String>) -> ExitCode {
    match outcome {
        Ok(n) => {
            println!("{n}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

/// Runs this program again as `child <extra>` and returns its exit code, stdout, stderr.
fn child(extra: &[&str]) -> (Option<i32>, String, String) {
    let out = Command::new(std::env::current_exe().unwrap())
        .arg("child")
        .args(extra)
        .output()
        .unwrap();
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() == Some("child") {
        return finish(run(args.next()));
    }
    assert_eq!(child(&["84"]), (Some(0), "42\n".into(), String::new()));
    // The failing cases: code 2, and the message on standard error only.
    assert_eq!(
        child(&["abc"]),
        (Some(2), String::new(), "not a number: abc\n".into())
    );
    assert_eq!(
        child(&[]),
        (Some(2), String::new(), "missing number\n".into())
    );

    println!("all checks passed");
    ExitCode::SUCCESS
}
