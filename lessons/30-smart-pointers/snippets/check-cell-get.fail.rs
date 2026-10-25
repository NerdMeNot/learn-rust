use std::cell::Cell;

fn main() {
    let name = Cell::new(String::from("x"));
    let copy = name.get();
    println!("{copy}");
}
