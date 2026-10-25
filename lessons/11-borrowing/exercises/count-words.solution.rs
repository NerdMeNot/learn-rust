/// Count the words in `text`. Words are separated by any whitespace: spaces, tabs,
/// newlines, or several in a row. The function only reads the text, so it must borrow it.
fn count_words(text: &str) -> usize {
    let mut count = 0;
    for _word in text.split_whitespace() {
        count += 1;
    }
    count
}

fn main() {
    // A &str literal works.
    assert_eq!(count_words("the quick brown fox"), 4);

    // So does a borrowed String, and the String is still ours afterwards.
    let line = String::from("  one \t two\nthree  ");
    assert_eq!(count_words(&line), 3);
    assert_eq!(line.len(), 19);

    // So does part of a String.
    assert_eq!(count_words(&line[6..]), 2);

    // Nothing to count.
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("   \n\t "), 0);

    // Not English: split_whitespace works on characters, not bytes.
    assert_eq!(count_words("café crème 東京"), 3);

    println!("all checks passed");
}
