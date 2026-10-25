fn main() {
    let count: i32 = 3;
    let price: f64 = 2.5;
    let total = count as f64 * price; // you ask for the conversion, so you can see it
    println!("{total}");

    let name = String::from("ana");
    let copy = name.clone(); // a real copy of the text, and it's visible
    println!("{name} {copy}");
}
