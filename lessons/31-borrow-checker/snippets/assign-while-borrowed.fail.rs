fn main() {
    let mut position = String::from("start");
    let before = &position;
    position = String::from("middle");
    println!("moved from {before} to {position}");
}
