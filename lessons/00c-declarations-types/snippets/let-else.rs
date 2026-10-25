fn first_word_len(text: &str) -> usize {
    // `split_once` gives `Some((before, after))`, or `None` when there is no space
    let Some((word, _)) = text.split_once(' ') else {
        return text.len(); // the `else` block must leave: return, continue, break or panic
    };
    word.len()
}

fn main() {
    println!(
        "first_word_len: {} {}",
        first_word_len("hello world"),
        first_word_len("hi")
    );
    for text in ["1", "x", "3"] {
        let Ok(n) = text.parse::<i32>() else {
            continue; // skip the texts that are not numbers
        };
        println!("number: {n}");
    }
}
