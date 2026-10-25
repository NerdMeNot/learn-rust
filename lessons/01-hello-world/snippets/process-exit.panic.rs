fn leave() {
    println!("leaving from a nested call");
    std::process::exit(4);
}

fn main() {
    leave();
    println!("not printed");
}
