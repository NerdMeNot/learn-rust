use std::process::{Command, Output};

const SOURCE: &str = r#"use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

macro_rules! sum {
    ($x:expr) => {
        $x
    };
    ($head:expr, $($tail:expr),+) => {
        $head + sum!($($tail),+)
    };
}

fn main() {
    let ages = hashmap! { "Ada" => 36, "Linus" => 28 };
    let total = sum!(1, 2, 3);
    println!("{} entries, total {total}", ages.len());
}
"#;

// The Rust below is only a harness: it saves SOURCE and asks rustc for the expansion.
// -Zunpretty=expanded is a nightly flag; RUSTC_BOOTSTRAP=1 makes this stable compiler accept it.
fn rustc(bootstrap: bool) -> Output {
    let path = std::env::temp_dir().join("expand-demo.rs");
    std::fs::write(&path, SOURCE).unwrap();
    let mut command = Command::new("rustc");
    command
        .args(["--edition", "2024", "-Zunpretty=expanded"])
        .arg(&path);
    if bootstrap {
        command.env("RUSTC_BOOTSTRAP", "1");
    }
    let out = command.output().unwrap();
    std::fs::remove_file(&path).unwrap();
    out
}

fn main() {
    let refused = String::from_utf8_lossy(&rustc(false).stderr).into_owned();
    println!(
        "without RUSTC_BOOTSTRAP: {}",
        refused.lines().next().unwrap()
    );

    let text = String::from_utf8_lossy(&rustc(true).stdout).into_owned();
    // The macro definitions stay in the output; the interesting part is `main`.
    let main_at = text.find("fn main()").unwrap();
    println!(
        "with RUSTC_BOOTSTRAP=1, from `fn main`:\n{}",
        &text[main_at..]
    );
}
