fn main() {
    // These return a new String and leave the original alone.
    let title = "Rust for Gophers";
    println!("replace: {}", title.replace("Gophers", "everyone"));
    println!("to_lowercase: {}", title.to_lowercase());
    println!("to_uppercase: {}", title.to_uppercase());
    println!("straße upper: {}", "straße".to_uppercase()); // ß becomes SS

    // These change a String in place, so it must be `mut`.
    let mut s = String::from("hello world");
    s.insert(0, '>');
    s.insert_str(1, "> ");
    println!("after insert: {s:?}");
    println!("pop returned: {:?}", s.pop()); // removes and returns the last char
    s.truncate(5); // keep the first 5 bytes
    println!("after truncate(5): {s:?}");
    s.retain(|c| c != '>'); // keep only what the closure says yes to
    println!("after retain: {s:?}");
    s.clear();
    println!("after clear: {s:?}");

    // Comparing.
    println!("\"Rust\" == \"rust\": {}", "Rust" == "rust");
    println!("ignoring case: {}", "Rust".eq_ignore_ascii_case("rust"));
    println!("\"apple\" vs \"banana\": {:?}", "apple".cmp("banana"));

    let mut names = vec!["bo", "Cy", "ana"];
    names.sort(); // byte order: every capital comes before every lowercase letter
    println!("sort: {names:?}");
    names.sort_by_key(|n| n.to_lowercase()); // what a person expects
    println!("sort_by_key(lowercase): {names:?}");
}
