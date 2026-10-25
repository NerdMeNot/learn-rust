fn main() {
    let names = vec![String::from("ana"), String::from("bo")];
    for n in names {
        println!("hi {n}");
    }
    println!("{} names", names.len());
}
