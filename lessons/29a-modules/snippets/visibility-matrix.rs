use std::fs;
use std::path::Path;
use std::process::Command;

// Compiles a small library once for each visibility, with a call to `f` from each place,
// and reports which calls the compiler accepted. Each cell is the result of a real
// `rustc` run, not typed in.
const PLACES: [&str; 7] = [
    "in a",
    "child",
    "parent",
    "sibling",
    "cousin",
    "root",
    "other crate",
];

// `f` lives in `outer::mid::a`. The calls are on the lines listed in CALL_LINES.
fn library(vis: &str) -> String {
    format!(
        "pub mod outer {{
    pub mod mid {{
        pub mod a {{
            {vis}fn f() {{}}
            fn inside() {{ f(); }}
            pub mod child {{
                fn probe() {{ super::f(); }}
            }}
        }}
        fn parent() {{ a::f(); }}
        pub mod b {{
            fn probe() {{ super::a::f(); }}
        }}
    }}
    pub mod cousin {{
        fn probe() {{ super::mid::a::f(); }}
    }}
}}
fn root() {{ outer::mid::a::f(); }}
"
    )
}
const CALL_LINES: [usize; 6] = [5, 7, 10, 12, 16, 19];

fn rustc(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("rustc")
        .current_dir(dir)
        .args([
            "--edition",
            "2024",
            "-A",
            "warnings",
            "--error-format=short",
        ])
        .args(args)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stderr).into_owned()
}

// "ok", or the error code of the first error in the compiler's output.
fn verdict(errors: &str) -> String {
    match errors.find("error[") {
        Some(i) => errors[i + 6..i + 11].to_string(),
        None => "ok".to_string(),
    }
}

fn row(name: &str, cells: &[String]) {
    let mut line = format!("{name:<22}");
    for c in cells {
        line += &format!(" {c:<8}");
    }
    println!("{}", line.trim_end());
}

fn main() {
    let dir = std::env::temp_dir().join("modules-matrix");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    row("visibility", &PLACES.map(String::from));
    for vis in [
        "",
        "pub(super) ",
        "pub(in crate::outer) ",
        "pub(crate) ",
        "pub ",
    ] {
        // Calls from inside the crate.
        fs::write(dir.join("calls.rs"), library(vis)).unwrap();
        let errors = rustc(&dir, &["--crate-type=lib", "--emit=metadata", "calls.rs"]);
        let mut cells: Vec<String> = Vec::new();
        for line in CALL_LINES {
            let hit = errors
                .lines()
                .find(|l| l.contains(&format!("calls.rs:{line}:")));
            cells.push(hit.map_or("ok".to_string(), verdict));
        }
        // A call from another crate needs a library that has no errors of its own.
        let clean =
            format!("pub mod outer {{ pub mod mid {{ pub mod a {{ {vis}fn f() {{}} }} }} }}\n");
        fs::write(dir.join("demo.rs"), clean).unwrap();
        rustc(&dir, &["--crate-type=lib", "--emit=metadata", "demo.rs"]);
        fs::write(
            dir.join("user.rs"),
            "fn main() { demo::outer::mid::a::f(); }\n",
        )
        .unwrap();
        let errors = rustc(
            &dir,
            &[
                "--emit=metadata",
                "--extern",
                "demo=libdemo.rmeta",
                "user.rs",
            ],
        );
        cells.push(verdict(&errors));

        let name = if vis.is_empty() {
            "(no modifier)"
        } else {
            vis.trim()
        };
        row(name, &cells);
    }
}
