fn main() {
    let name = String::from("ferris");
    let consume = || drop(name);
    consume();
    println!("{name}");
}
