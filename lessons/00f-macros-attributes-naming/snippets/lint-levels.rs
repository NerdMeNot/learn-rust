use std::fs;
use std::process::Command;

const BODY: &str = "fn main() {\n    let count = 3;\n}\n";

// Compile `source` with the plain `rustc` command and return what it printed.
fn rustc(source: &str) -> String {
    let dir = std::env::temp_dir().join("lint-levels-demo");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("demo.rs"), source).unwrap();
    let out = Command::new("rustc")
        .args(["--edition", "2024", "demo.rs"])
        .current_dir(&dir)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn main() {
    println!("--- no attribute (the default level) ---");
    print!("{}", rustc(BODY));

    println!("--- #[allow(unused_variables)] ---");
    let out = rustc(&format!("#[allow(unused_variables)]\n{BODY}"));
    println!(
        "{}",
        if out.is_empty() {
            "(nothing printed)"
        } else {
            &out
        }
    );

    println!("--- #![deny(unused_variables)] ---");
    let out = rustc(&format!("#![deny(unused_variables)]\n{BODY}"));
    println!("{}", out.lines().next().unwrap());
}
