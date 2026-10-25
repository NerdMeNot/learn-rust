fn main() {
    let name = String::from("ana");
    let mut guests = Vec::new();
    for _ in 0..3 {
        guests.push(name); // the first pass moves name away
    }
    println!("{guests:?}");
}
