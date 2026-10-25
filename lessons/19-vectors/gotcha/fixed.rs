fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    let first = &names[0]; // borrow it; `names[0].clone()` would copy it
    println!("{first} of {}", names.len());
}
