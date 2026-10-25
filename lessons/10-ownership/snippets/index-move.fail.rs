fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    let first = names[0]; // move a String out of the Vec?
    println!("{first}");
}
