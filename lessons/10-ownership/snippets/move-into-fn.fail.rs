fn count_words(text: String) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let title = String::from("Ownership and Moves");
    println!("words: {}", count_words(title));
    println!("title: {title}");
}
