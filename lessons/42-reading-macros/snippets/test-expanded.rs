use std::process::Command;

const SOURCE: &str = "#[test]
fn adds() {
    assert_eq!(1 + 1, 2);
}
";

// A harness: `--test` is the flag `cargo test` passes to rustc; RUSTC_BOOTSTRAP=1 as above.
fn expand(test_mode: bool) -> String {
    let dir = std::env::temp_dir();
    std::fs::write(dir.join("test-demo.rs"), SOURCE).unwrap();
    let mut command = Command::new("rustc");
    command.args([
        "--edition",
        "2024",
        "--crate-type",
        "lib",
        "-Zunpretty=expanded",
    ]);
    if test_mode {
        command.arg("--test");
    }
    let out = command
        .arg("test-demo.rs")
        .current_dir(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .output()
        .unwrap();
    std::fs::remove_file(dir.join("test-demo.rs")).unwrap();
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn main() {
    let text = expand(true);
    // From the generated constant to your function, then the generated `main`.
    let start = text.find("extern crate test;").unwrap();
    let body = text.find("fn adds()").unwrap();
    let main_at = text.find("#[rustc_main]").unwrap();
    println!("--- with --test ---");
    println!("{}", text[start..body].trim_end());
    println!("fn adds() {{ ... }}   // your function, with assert_eq! expanded");
    println!("{}", text[main_at..].trim_end());

    println!("--- without --test ---");
    println!(
        "expansion mentions `adds`: {}",
        expand(false).contains("adds")
    );
}
