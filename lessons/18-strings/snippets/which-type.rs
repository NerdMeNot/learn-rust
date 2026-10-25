// A function that only reads text takes &str. It then accepts every kind of string.
fn shout(text: &str) -> String {
    text.to_uppercase()
}

// A struct that keeps text owns it: String.
struct User {
    name: String,
}

fn main() {
    let literal = "hello"; // &'static str: baked into the program
    let owned = String::from("world"); // String: on the heap, yours to change
    let user = User {
        name: String::from("ana"),
    };

    println!("{}", shout(literal));
    println!("{}", shout(&owned)); // &String turns into &str by itself
    println!("{}", shout(&user.name));
    println!("{}", shout(&owned[1..3])); // a slice of a String is a &str too
}
