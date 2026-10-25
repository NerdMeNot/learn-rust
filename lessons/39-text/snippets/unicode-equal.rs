fn main() {
    // Two ways to write "é": one char, or an e followed by a combining accent.
    let one = "caf\u{e9}";
    let two = "cafe\u{301}";
    println!("{one} and {two} look alike");
    println!("equal:      {}", one == two);
    println!("bytes:      {} and {}", one.len(), two.len());
    println!(
        "chars:      {} and {}",
        one.chars().count(),
        two.chars().count()
    );
    println!("one: {one:?}");
    println!("two: {two:?}");
}
