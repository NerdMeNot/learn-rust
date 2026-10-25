fn find(key: &str) -> Option<&'static str> {
    if key == "a" { Some("apple") } else { None }
}

fn main() {
    let s = find("b");
    println!("{}", s.to_uppercase());
}
