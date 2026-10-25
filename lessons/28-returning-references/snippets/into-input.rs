fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let text = String::from("hello world");
    let w = first_word(&text);
    let start = text.as_ptr() as usize;
    println!("w: {w:?}");
    println!("w starts {} bytes into text", w.as_ptr() as usize - start);
    println!(
        "w.len(): {}, text.len(): {}, text.capacity(): {}",
        w.len(),
        text.len(),
        text.capacity()
    );
    let again = first_word(&text[6..]);
    println!(
        "again: {again:?}, starts {} bytes into text",
        again.as_ptr() as usize - start
    );
}
