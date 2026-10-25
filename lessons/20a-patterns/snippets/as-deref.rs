fn greeting(nick: &Option<String>) -> String {
    // as_deref turns &Option<String> into Option<&str>, which string patterns can test
    match nick.as_deref() {
        Some("ana") => String::from("hello, ana"),
        Some(other) => format!("hi, {other}"),
        None => String::from("hello, stranger"),
    }
}

fn main() {
    for nick in [Some(String::from("ana")), Some(String::from("bo")), None] {
        println!("{nick:?} -> {}", greeting(&nick));
    }
}
