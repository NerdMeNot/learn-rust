use std::fs::File;

fn main() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    println!("{}", text.lines().count());
}
