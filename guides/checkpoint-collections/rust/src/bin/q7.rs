fn main() {
    let name = String::from("Ada");
    let consume = || drop(name);
    consume();
    consume();
}
