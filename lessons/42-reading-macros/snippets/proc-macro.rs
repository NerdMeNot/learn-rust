use std::fs;
use std::path::Path;
use std::process::Command;

// A harness: it writes a two-crate workspace to a temporary folder and builds it with Cargo.
// No dependencies: `proc_macro` comes with the compiler.
const WORKSPACE: &str = "[workspace]\nmembers = [\"shout\", \"app\"]\nresolver = \"3\"\n";

// The crate that holds the macros. `proc-macro = true` is what makes it one.
const SHOUT_TOML: &str = "[package]\nname = \"shout\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\nproc-macro = true\n";

const SHOUT_LIB: &str = r#"use proc_macro::TokenStream;

// make_answer!(): ignore the input and write a function.
#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// #[derive(TypeName)]: read the name after `struct`, write an impl block.
#[proc_macro_derive(TypeName)]
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    let name = text.split_whitespace().nth(1).unwrap().trim_end_matches(';');
    format!("impl {name} {{ fn type_name() -> &'static str {{ \"{name}\" }} }}")
        .parse()
        .unwrap()
}

// #[show_streams(...)]: print what arrives, at compile time, and return the item unchanged.
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}
"#;

// The crate that uses them.
const APP_TOML: &str = "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshout = { path = \"../shout\" }\n";

const APP_MAIN: &str = r#"use shout::{TypeName, make_answer, show_streams};

make_answer!();

#[derive(TypeName)]
struct Pet;

#[show_streams(loud, times = 2)]
fn greet() -> &'static str {
    "hi"
}

fn main() {
    println!("Pet::type_name() = {}", Pet::type_name());
    println!("greet() = {}", greet());
    println!("answer() = {}", answer());
}
"#;

fn main() {
    let root = std::env::temp_dir().join("proc-macro-demo");
    let _ = fs::remove_dir_all(&root);
    for (file, text) in [
        ("Cargo.toml", WORKSPACE),
        ("shout/Cargo.toml", SHOUT_TOML),
        ("shout/src/lib.rs", SHOUT_LIB),
        ("app/Cargo.toml", APP_TOML),
        ("app/src/main.rs", APP_MAIN),
    ] {
        let path = root.join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    let build = Command::new("cargo")
        .args(["build", "--quiet", "--offline", "--package", "app"])
        .env("CARGO_TARGET_DIR", root.join("target"))
        .current_dir(&root)
        .output()
        .unwrap();
    println!("--- cargo build: success = {} ---", build.status.success());
    print!("{}", String::from_utf8_lossy(&build.stdout));
    print!("{}", String::from_utf8_lossy(&build.stderr));

    println!("--- target/debug/app ---");
    let run = Command::new(Path::new(&root).join("target/debug/app"))
        .output()
        .unwrap();
    print!("{}", String::from_utf8_lossy(&run.stdout));
    let _ = fs::remove_dir_all(&root);
}
