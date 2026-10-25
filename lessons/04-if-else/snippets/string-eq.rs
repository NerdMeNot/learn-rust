fn main() {
    let a = String::from("rust");
    let b = "rust";
    // == on text compares the characters, not where they live
    println!("a == b: {}", a == b);
}
