// By default an ignored Result is a warning. This line turns it into an error,
// so the page can show the message.
#![deny(unused_must_use)]

fn save(name: &str) -> Result<(), String> {
    if name.is_empty() {
        Err("empty name".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    save("");
    // Ignoring on purpose is allowed. Say so with let _ =.
    let _ = save("");
    println!("done");
}
