// 'a says: the result borrows from both inputs, so it is valid only while both are.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let first = String::from("lifetimes");
    let second = String::from("regions");
    let result = longest(&first, &second);
    println!("longest: {result}");
}
