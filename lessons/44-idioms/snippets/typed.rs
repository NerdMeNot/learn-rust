use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Read,
    Write,
}

impl FromStr for Mode {
    type Err = String;

    /// Text is turned into a `Mode` once, at the edge of the program.
    fn from_str(s: &str) -> Result<Mode, String> {
        match s {
            "read" => Ok(Mode::Read),
            "write" => Ok(Mode::Write),
            other => Err(format!("unknown mode {other:?}")),
        }
    }
}

fn open_flags(mode: Mode) -> &'static str {
    match mode {
        Mode::Read => "O_RDONLY",
        Mode::Write => "O_WRONLY",
    }
}

fn main() {
    for text in ["read", "write", "wrtie"] {
        match text.parse::<Mode>() {
            Ok(mode) => println!("{text} -> {}", open_flags(mode)),
            Err(e) => println!("{text} -> {e}"),
        }
    }
    println!("{}", open_flags(Mode::Write));
}
