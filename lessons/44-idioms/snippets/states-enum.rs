/// One variant per state; each carries only the data that state has.
#[derive(Debug)]
enum Download {
    Running,
    Done { bytes: u64 },
    Failed { reason: String },
}

fn describe(d: &Download) -> String {
    match d {
        Download::Running => "running".to_string(),
        Download::Done { bytes } => format!("finished, {bytes} bytes"),
        Download::Failed { reason } => format!("failed: {reason}"),
    }
}

fn main() {
    let states = [
        Download::Running,
        Download::Done { bytes: 512 },
        Download::Failed {
            reason: "timeout".to_string(),
        },
    ];
    for s in &states {
        println!("{}", describe(s));
    }
    // "done and failed at once" has no spelling: there is no variant that holds both.
}
