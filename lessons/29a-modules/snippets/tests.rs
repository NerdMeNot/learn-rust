use std::fs;
use std::path::Path;
use std::process::Command;

fn write(root: &Path, path: &str, text: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, text).unwrap();
}

// Runs one cargo command. `all` prints the test report lines; otherwise only the
// error lines. Test times and build folder hashes are cut, because they change.
fn cargo(dir: &Path, args: &[&str], all: bool) {
    println!("$ cargo {}", args.join(" "));
    // Through `sh`, so the compiler's and the test runner's lines keep their order.
    let out = Command::new("sh")
        .args(["-c", &format!("cargo {} 2>&1", args.join(" "))])
        .current_dir(dir)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let line = line.split("; finished in").next().unwrap();
        let line = line.split(" (target/").next().unwrap();
        let is_error = line.starts_with("error") && !line.contains("could not compile");
        let is_report = line.starts_with("test ")
            || line.trim_start().starts_with("Running")
            || line.contains("Doc-tests");
        if is_error || (all && is_report) {
            println!("{}", line.trim());
        }
    }
    println!("(exit status {})", out.status.code().unwrap());
}

const LIB: &str = r#"pub fn total(qty: u32) -> u32 {
    qty * 10 - qty * 10 * discount(qty) / 100
}

fn discount(qty: u32) -> u32 {
    if qty >= 10 { 20 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_discount_starts_at_ten() {
        assert_eq!(discount(9), 0);
        assert_eq!(discount(10), 20);
    }
}
"#;

const API_TEST: &str = r#"#[test]
fn twelve_items() {
    assert_eq!(shop::total(12), 96);
}
"#;

fn main() {
    let dir = std::env::temp_dir().join("modules-tests").join("shop");
    let _ = fs::remove_dir_all(&dir);
    let toml = "[package]\nname = \"shop\"\nversion = \"0.1.0\"\nedition = \"2024\"\n";
    write(&dir, "Cargo.toml", toml);
    write(&dir, "src/lib.rs", LIB);
    write(&dir, "tests/api.rs", API_TEST);
    cargo(&dir, &["test"], true);

    // An integration test is a separate crate: it sees only what is `pub`.
    let peek = API_TEST.replace("shop::total(12)", "shop::discount(12)");
    write(&dir, "tests/api.rs", &peek);
    cargo(&dir, &["test"], false);

    // Code under #[cfg(test)] is not compiled for a normal build.
    write(&dir, "tests/api.rs", API_TEST);
    let broken = LIB.replace("assert_eq!(discount(9), 0);", "let _x: u32 = \"nine\";");
    write(&dir, "src/lib.rs", &broken);
    cargo(&dir, &["build"], false);
    cargo(&dir, &["test"], false);
}
