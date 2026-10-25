fn main() {
    let owned: String = String::from("héllo");   // owns the text, can grow
    let view: &str = &owned[0..1];                // a view into it, no copy

    let mut s = String::from("go");
    s.push_str(" + rust");

    println!("{owned} {view} {s}");
    println!("bytes={} chars={}", owned.len(), owned.chars().count());
    for c in owned.chars() {
        print!("[{c}]");
    }
    println!();
}
