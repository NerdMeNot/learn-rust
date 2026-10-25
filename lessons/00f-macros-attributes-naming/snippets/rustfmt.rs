use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let messy = r#"fn area(w:u32,h:u32)->u32{
let a=w*h;
  if a>100 {println!("big");}
        a
}
struct Point{x:i32,y:i32}
"#;
    // With no file name, `rustfmt` reads source from stdin and writes the result to stdout.
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(messy.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    println!(
        "--- before ---\n{messy}--- after ---\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
}
