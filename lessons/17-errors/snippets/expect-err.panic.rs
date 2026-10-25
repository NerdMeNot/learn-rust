fn main() {
    let text = "forty";
    // expect is unwrap with your own message in front of the error.
    let age: u32 = text.parse().expect("age should be a whole number");
    println!("age: {age}");
}
