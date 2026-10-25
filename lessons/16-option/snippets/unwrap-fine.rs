fn main() {
    // The text is a literal in this file, and it has a colon. Checking it by hand
    // proves split_once cannot return None, so expect states why.
    let setting = "mode: fast";
    let (key, value) = setting.split_once(": ").expect("the literal contains ': '");
    println!("key: {key}, value: {value}");

    // The list is built just above and is never empty.
    let scores = vec![72, 95, 88];
    let first = scores.first().expect("scores has three items");
    println!("first score: {first}");
}
