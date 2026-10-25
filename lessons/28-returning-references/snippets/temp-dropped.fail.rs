fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let w = first_word(&String::from("hello world"));
    println!("w: {w}");
}
