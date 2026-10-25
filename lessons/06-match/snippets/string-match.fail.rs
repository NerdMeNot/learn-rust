fn main() {
    let cmd = String::from("go");
    match cmd {
        "go" => println!("moving"),
        _ => println!("unknown command"),
    }
}
