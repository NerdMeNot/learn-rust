/// Three fields that must agree, and nothing makes them.
#[derive(Debug)]
struct Download {
    done: bool,
    error: Option<String>,
    bytes: Option<u64>,
}

fn describe(d: &Download) -> String {
    if d.done {
        format!("finished, {} bytes", d.bytes.unwrap_or(0))
    } else if let Some(e) = &d.error {
        format!("failed: {e}")
    } else {
        "running".to_string()
    }
}

fn main() {
    let fine = Download {
        done: true,
        error: None,
        bytes: Some(512),
    };
    // Compiles. Is it finished or failed? `describe` quietly picks one.
    let contradiction = Download {
        done: true,
        error: Some("timeout".to_string()),
        bytes: None,
    };
    println!("{}", describe(&fine));
    println!("{}", describe(&contradiction));
}
