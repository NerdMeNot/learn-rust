// The longest prefix of `text` that fits in `max` bytes and ends on a char boundary.
fn fit(text: &str, max: usize) -> &str {
    &text[..text.floor_char_boundary(max)]
}

fn main() {
    let text = "héllo 🦀";
    println!("len: {} bytes", text.len());
    for max in [0, 1, 2, 3, 8, 9, 100] {
        println!("fit({max:>3}): {:?}", fit(text, max));
    }
    println!("is_char_boundary(2): {}", text.is_char_boundary(2));
    println!("is_char_boundary(3): {}", text.is_char_boundary(3));

    // A boundary between chars is not always a boundary between letters:
    // a flag is two chars (regional indicators), 4 bytes each.
    let flag = "🇵🇹";
    println!("chars in the flag: {}", flag.chars().count());
    println!("fit(flag, 4): {:?}", fit(flag, 4));
}
