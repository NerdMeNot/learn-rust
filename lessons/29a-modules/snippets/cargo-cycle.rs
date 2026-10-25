use std::fs;
use std::path::Path;
use std::process::Command;

fn write(root: &Path, path: &str, text: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, text).unwrap();
}

fn main() {
    let root = std::env::temp_dir().join("modules-cargo-cycle");
    let _ = fs::remove_dir_all(&root);
    // Two packages, each listing the other as a dependency.
    for (name, other) in [("a", "b"), ("b", "a")] {
        let toml = format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{other} = {{ path = \"../{other}\" }}\n"
        );
        write(&root, &format!("{name}/Cargo.toml"), &toml);
        write(&root, &format!("{name}/src/lib.rs"), "");
    }
    let out = Command::new("cargo")
        .args(["check", "-q"])
        .current_dir(root.join("a"))
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stderr);
    for line in text.lines().filter(|l| l.starts_with("error")) {
        // Leave out the temporary folder the package sits in.
        println!(
            "{}",
            line.replace(&format!(" ({})", root.join("a").display()), "")
        );
    }
}
