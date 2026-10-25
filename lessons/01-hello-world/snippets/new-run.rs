use std::fs;
use std::path::Path;
use std::process::Command;

// Runs `cargo <args>` in `dir` and prints what a terminal would show. Two things are
// trimmed: the build time ("in 0.06s") and the folder the project lives in.
fn cargo(dir: &Path, args: &[&str]) {
    println!("$ cargo {}", args.join(" "));
    let out = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stderr).into_owned() + &String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let line = line.replace(dir.parent().unwrap().to_str().unwrap(), "~/projects");
        match line.find(" target(s) in ") {
            Some(i) => println!("{} target(s)", &line[..i]),
            None => println!("{line}"),
        }
    }
}

// The visible entries of a folder, folders marked with a slash.
fn ls(dir: &Path) -> String {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| {
            let slash = if e.path().is_dir() { "/" } else { "" };
            format!("{}{slash}", e.file_name().to_string_lossy())
        })
        .collect();
    names.sort();
    names.join("  ")
}

fn main() {
    let root = std::env::temp_dir().join("hw-new-run");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    cargo(&root, &["new", "hello"]);
    let dir = root.join("hello");
    println!("ls hello: {}", ls(&dir));
    println!(
        "--- Cargo.toml ---\n{}",
        fs::read_to_string(dir.join("Cargo.toml")).unwrap()
    );
    println!(
        "--- .gitignore ---\n{}",
        fs::read_to_string(dir.join(".gitignore")).unwrap()
    );
    println!(
        "--- src/main.rs ---\n{}",
        fs::read_to_string(dir.join("src/main.rs")).unwrap()
    );

    cargo(&dir, &["run"]);
    println!("ls hello: {}", ls(&dir));
    println!("ls hello/target: {}", ls(&dir.join("target")));
    println!("ls hello/target/debug: {}", ls(&dir.join("target/debug")));
}
