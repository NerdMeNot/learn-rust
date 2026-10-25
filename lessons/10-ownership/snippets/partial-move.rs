#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let user = User {
        id: 7,
        name: String::from("ana"),
        email: String::from("ana@example.com"),
    };

    let name = user.name; // move one field out
    println!("moved out: {name}");

    // The fields that weren't moved are still usable...
    println!("still here: {} {}", user.id, user.email);

    // ...but user as a whole isn't: `println!("{user:?}")` would not compile.

    // Destructuring moves every field out at once, into new variables.
    let other = User {
        id: 8,
        name: String::from("bo"),
        email: String::from("bo@example.com"),
    };
    let User { id, email, .. } = other;
    println!("destructured: {id} {email}");
}
