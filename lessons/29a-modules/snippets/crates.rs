use std::fs;
use std::path::Path;
use std::process::Command;

fn write(root: &Path, path: &str, text: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, text).unwrap();
}

// Runs a cargo command in `dir`; prints its output, or only the error lines.
fn cargo(dir: &Path, args: &[&str], errors_only: bool) {
    println!("$ cargo {}", args.join(" "));
    let out = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    for line in text.lines() {
        let is_error = line.starts_with("error") && !line.contains("could not compile");
        if !errors_only || is_error {
            println!("{line}");
        }
    }
}

const UTIL_TOML: &str = r#"[package]
name = "my-util"
version = "0.1.0"
edition = "2024"
"#;

const APP_TOML: &str = r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
my-util = { path = "../my-util" }
"#;

const UTIL_LIB: &str = r#"pub fn shout(s: &str) -> String {
    s.to_uppercase()
}
"#;

const APP_LIB: &str = r#"pub fn greet() -> String {
    my_util::shout(&secret())
}

pub(crate) fn secret() -> String {
    String::from("hello")
}
"#;

fn main() {
    let root = std::env::temp_dir().join("modules-crates");
    let _ = fs::remove_dir_all(&root);
    let app = root.join("app");
    write(&root, "my-util/Cargo.toml", UTIL_TOML);
    write(&root, "my-util/src/lib.rs", UTIL_LIB);
    write(&root, "app/Cargo.toml", APP_TOML);
    write(&root, "app/src/lib.rs", APP_LIB);
    let main_rs = |body: &str| format!("fn main() {{\n    println!(\"{{}}\", {body});\n}}\n");
    write(&root, "app/src/main.rs", &main_rs("app::greet()"));
    write(&root, "app/src/bin/tool.rs", &main_rs("app::greet()"));

    cargo(&app, &["build", "-q"], false);
    let mut built: Vec<String> = fs::read_dir(app.join("target/debug"))
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('.') && !n.ends_with(".d"))
        .collect();
    built.sort();
    println!("target/debug (files, not counting .d): {}", built.join(" "));
    cargo(&app, &["run", "-q", "--bin", "app"], false);
    cargo(&app, &["run", "-q", "--bin", "tool"], false);

    // main.rs is a different crate from lib.rs, so `pub(crate)` stops at lib.rs.
    write(&root, "app/src/main.rs", &main_rs("app::secret()"));
    cargo(&app, &["build", "-q"], true);
    // And `crate::` in main.rs means main.rs's own crate.
    write(&root, "app/src/main.rs", &main_rs("crate::greet()"));
    cargo(&app, &["build", "-q"], true);
}
