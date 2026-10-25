use std::process::Command;

const POINT: &str = "#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
";

const MODE: &str = "#[derive(Default)]
enum Mode {
    #[default]
    Fast,
    Slow,
}
";

// A harness: it asks rustc (RUSTC_BOOTSTRAP=1, as above) for the source after macros ran.
fn expand(source: &str) -> String {
    let path = std::env::temp_dir().join("derive-demo.rs");
    std::fs::write(&path, source).unwrap();
    let out = Command::new("rustc")
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "-Zunpretty=expanded",
        ])
        .arg(&path)
        .env("RUSTC_BOOTSTRAP", "1")
        .output()
        .unwrap();
    std::fs::remove_file(&path).unwrap();
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    // Skip the prelude lines every expansion starts with.
    text.lines().skip(4).collect::<Vec<_>>().join("\n")
}

fn main() {
    println!("--- you write ---\n{POINT}");
    println!("--- rustc sees ---\n{}\n", expand(POINT));
    println!("--- you write ---\n{MODE}");
    println!("--- rustc sees ---\n{}", expand(MODE));
}
