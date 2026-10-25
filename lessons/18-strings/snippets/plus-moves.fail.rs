fn main() {
    let hello = String::from("Hello, ");
    let world = String::from("world");
    let greeting = hello + &world;
    println!("{greeting}");
    println!("{hello}"); // hello was moved into greeting
}
