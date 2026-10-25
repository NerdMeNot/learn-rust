// Backs "an array of Strings is moved like a Vec". Never shown.
fn main() {
    let names = [String::from("ana"), String::from("bo")];
    for n in names {
        println!("hi {n}");
    }
    println!("{} names", names.len());
}
