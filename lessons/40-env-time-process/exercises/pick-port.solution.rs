use std::env::{self, VarError};
use std::ffi::OsString;

/// The port to listen on: the first argument after the program name if it is a valid port
/// number, else the value of `RBC_PORT` (passed in as `var`) if it is one, else 8080.
/// An empty or non-numeric value counts as not given.
fn port(args: &[String], var: Result<String, VarError>) -> u16 {
    args.get(1)
        .and_then(|arg| arg.parse().ok())
        .or_else(|| var.ok().and_then(|value| value.parse().ok()))
        .unwrap_or(8080)
}

fn main() {
    let args = |list: &[&str]| list.iter().map(|a| a.to_string()).collect::<Vec<String>>();
    let set = |value: &str| Ok(value.to_string());
    let unset = || Err(VarError::NotPresent);

    assert_eq!(port(&args(&["prog"]), unset()), 8080);
    assert_eq!(port(&args(&["prog", "9000"]), unset()), 9000);
    assert_eq!(port(&args(&["prog"]), set("7000")), 7000);
    // an argument beats the variable
    assert_eq!(port(&args(&["prog", "9000"]), set("7000")), 9000);
    // an empty variable is not a port, and neither is an argument that is not a number
    assert_eq!(port(&args(&["prog"]), set("")), 8080);
    assert_eq!(port(&args(&["prog", "abc"]), set("7000")), 7000);
    assert_eq!(port(&args(&["prog", "abc"]), unset()), 8080);
    // 70000 does not fit in a u16
    assert_eq!(port(&args(&["prog", "70000"]), set("7000")), 7000);
    // the program name is not an argument, even when it looks like a number
    assert_eq!(port(&args(&["4000"]), unset()), 8080);
    assert_eq!(port(&args(&["4000"]), set("7000")), 7000);
    // a variable that is not valid UTF-8
    assert_eq!(
        port(
            &args(&["prog"]),
            Err(VarError::NotUnicode(OsString::from("x")))
        ),
        8080
    );
    // the real thing: this process has no arguments, and no such variable
    let real: Vec<String> = env::args().collect();
    assert_eq!(port(&real, env::var("RBC_SURELY_NOT_SET_9F3A")), 8080);

    println!("all checks passed");
}
