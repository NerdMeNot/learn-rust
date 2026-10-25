use std::fs;
use std::path::Path;
use std::process::Command;

// Builds a small project from `files` with `cargo check` and prints each file's
// `mod` lines, then whether it compiled and, if not, what the compiler said.
fn case(n: u32, title: &str, files: &[(&str, &str)]) {
    let dir = std::env::temp_dir().join(format!("modules-lookup-{n}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"lookup\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    println!("{n}. {title}");
    for (path, text) in files {
        let full: &Path = &dir.join(path);
        fs::create_dir_all(full.parent().unwrap()).unwrap();
        fs::write(full, text).unwrap();
        let mods: Vec<&str> = text
            .lines()
            .filter(|l| l.contains("mod ") || l.contains("include!"))
            .collect();
        let mods = if mods.is_empty() {
            "-".to_string()
        } else {
            mods.join(" ")
        };
        println!("  {path:<14} {mods}");
    }
    let out = Command::new("cargo")
        .args(["check", "-q"])
        .current_dir(&dir)
        .output()
        .unwrap();
    if out.status.success() {
        println!("  -> ok");
    }
    let text = String::from_utf8_lossy(&out.stderr);
    for line in text.lines() {
        if line.starts_with("error[") || line.contains("= help") {
            println!("  -> {}", line.trim());
        }
    }
}

// One line per case: a title, then (path, contents) for each file.
#[rustfmt::skip]
fn main() {
    let main = "mod a;\nfn main() {}\n";
    case(1, "a file next to the parent", &[("src/main.rs", main), ("src/a.rs", "")]);
    case(2, "a folder with mod.rs", &[("src/main.rs", main), ("src/a/mod.rs", "")]);
    case(3, "both of those", &[("src/main.rs", main), ("src/a.rs", ""), ("src/a/mod.rs", "")]);
    case(4, "neither", &[("src/main.rs", main)]);
    case(5, "a.rs declares a child", &[("src/main.rs", main), ("src/a.rs", "mod b;\n"), ("src/a/b.rs", "")]);
    case(6, "a/mod.rs declares a child", &[("src/main.rs", main), ("src/a/mod.rs", "mod b;\n"), ("src/a/b.rs", "")]);
    case(7, "the child file is beside a.rs", &[("src/main.rs", main), ("src/a.rs", "mod b;\n"), ("src/b.rs", "")]);
    case(8, "an inline module declares a file", &[("src/main.rs", "mod a { mod b; }\nfn main() {}\n"), ("src/a/b.rs", "")]);
    case(9, "a file nobody declares (it is not Rust)", &[("src/main.rs", main), ("src/a.rs", ""), ("src/extra.rs", "this is not Rust\n")]);
    case(10, "the same mod line twice", &[("src/main.rs", "mod a;\nmod a;\nfn main() {}\n"), ("src/a.rs", "")]);
    case(11, "include! instead of mod", &[("src/main.rs", "include!(\"extra.rs\");\nfn main() { extra(); }\n"), ("src/extra.rs", "fn extra() {}\n")]);
}
