use std::fs;
use std::process::Command;

const SOURCE: &str = r#"fn double(x: i32) -> i32 {
    return x * 2;
}

fn no_items(items: &[i32]) -> bool {
    items.len() == 0
}

fn main() {
    println!("{} {}", double(4), no_items(&[]));
}
"#;

fn main() {
    // A throwaway Cargo project, so that `cargo clippy` has something to check.
    let dir = std::env::temp_dir().join("clippy-demo");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    let manifest = "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n";
    fs::write(dir.join("Cargo.toml"), manifest).unwrap();
    fs::write(dir.join("src/main.rs"), SOURCE).unwrap();

    let out = Command::new("cargo")
        .arg("clippy")
        .current_dir(&dir)
        .output()
        .unwrap();
    for line in String::from_utf8_lossy(&out.stderr).lines() {
        if !line.trim_start().starts_with("Checking") && !line.trim_start().starts_with("Finished")
        {
            println!("{line}");
        }
    }
}
