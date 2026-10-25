fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let w = first_word(&text);
    text.clear();
    println!("w: {w}");
}
