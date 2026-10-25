fn main() {
    let names = vec!["ana", "bo"];
    println!("{:?}", names.get(5)); // None: the type says it may be missing
    let upper = names.get(5).map(|n| n.to_uppercase());
    println!("{}", upper.unwrap_or("none".to_string()));

    let port: Option<u16> = Some(0);
    println!("{}", port.unwrap_or(8080)); // only None falls back, like ??
}
