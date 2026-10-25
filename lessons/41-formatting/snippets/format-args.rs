use std::fmt;

// Takes the format string and its arguments, not a finished String.
fn log(level: &str, args: fmt::Arguments) {
    println!("[{level}] {args}");
}

macro_rules! info {
    ($($arg:tt)*) => { log("info", format_args!($($arg)*)) };
}

fn main() {
    log("warn", format_args!("disk at {}%", 91));
    info!("user {} logged in from {}", "ada", "10.0.0.7");
    info!("{:>6.2}|{:<6}|", 3.14159, "ab");
}
