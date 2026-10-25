fn make_greeting(name: &str) -> &str {
    let s = format!("hello {name}");
    &s
}

fn main() {
    println!("{}", make_greeting("rust"));
}
