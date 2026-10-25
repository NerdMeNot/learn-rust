fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    let upper: Vec<String> = names.into_iter().map(|s| s.to_uppercase()).collect();
    println!("{upper:?}");
    println!("{names:?}"); // into_iter() took the Vec
}
