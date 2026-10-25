fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    for &name in &names {
        println!("{name}");
    }
}
