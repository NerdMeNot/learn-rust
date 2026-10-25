fn main() {
    let path = "/home/ana/notes/rust.md";

    println!("len: {}", path.len()); // length in bytes
    println!("is_empty: {}", path.is_empty());
    println!("contains(\"notes\"): {}", path.contains("notes"));
    println!("starts_with('/'): {}", path.starts_with('/'));
    println!("ends_with(\".md\"): {}", path.ends_with(".md"));

    // find gives a byte position, wrapped in Option because it might not be there.
    println!("find('/'): {:?}", path.find('/'));
    println!("rfind('/'): {:?}", path.rfind('/'));
    println!("find(\"python\"): {:?}", path.find("python"));

    // Matching with a closure: the first digit, if any.
    let code = "room-42b";
    println!(
        "first digit at: {:?}",
        code.find(|c: char| c.is_ascii_digit())
    );

    // Count how often something appears.
    println!("\"an\" in banana: {}", "banana".matches("an").count());
}
