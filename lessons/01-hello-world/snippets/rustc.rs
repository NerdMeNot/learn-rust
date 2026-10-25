use std::fs;
use std::process::Command;

// Runs a command in `dir` and prints its exit code and the first lines it said.
fn run(dir: &std::path::Path, program: &str, args: &[&str]) {
    let out = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    println!(
        "$ {program} {}  (exit code {:?})",
        args.join(" "),
        out.status.code()
    );
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    for line in text.lines().take(2) {
        println!("{line}");
    }
}

fn main() {
    let dir = std::env::temp_dir().join("hw-rustc");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("main.rs"),
        "fn main() {\n    let async = 5;\n    println!(\"async: {async}\");\n}\n",
    )
    .unwrap();

    // No Cargo.toml here, so nothing tells rustc which edition to use.
    run(&dir, "rustc", &["main.rs"]);
    run(&dir, "./main", &[]);
    run(&dir, "rustc", &["--edition", "2024", "main.rs"]);
}
