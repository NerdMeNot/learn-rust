fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    // |&s| tries to take the String out from behind the reference.
    let copies: Vec<String> = names.iter().map(|&s| s).collect();
    println!("{copies:?}");
}
