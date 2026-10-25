fn shout(name: &str) -> &str {
    let upper = name.to_uppercase();
    &upper
}

fn main() {
    println!("{}", shout("rust"));
}
