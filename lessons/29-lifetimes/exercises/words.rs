/// An iterator over the whitespace-separated words of a text.
///
/// The starter yields owned `String`s, so it compiles. A view into the text is cheaper:
/// change the iterator so each word is a `&str` into the text.
struct Words<'a> {
    rest: &'a str,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Words<'a> {
        Words { rest: text }
    }
}

impl Iterator for Words<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        todo!("skip leading whitespace, take up to the next whitespace, keep the rest")
    }
}

/// Do the bytes of `piece` lie inside the bytes of `text`? A copy would not.
fn inside(piece: &str, text: &str) -> bool {
    let start = text.as_ptr() as usize;
    let at = piece.as_ptr() as usize;
    at >= start && at + piece.len() <= start + text.len()
}

fn main() {
    let text = String::from("  alpha  beta\tgamma\n");
    let words: Vec<_> = Words::new(&text).collect();
    assert_eq!(words, ["alpha", "beta", "gamma"]);
    for word in &words {
        assert!(inside(&*word, &text));
    }

    // The words outlive the iterator that found them.
    let first;
    {
        let mut iter = Words::new(&text);
        first = iter.next();
    }
    assert_eq!(first.as_deref(), Some("alpha"));

    // Trailing whitespace does not make an empty word.
    assert_eq!(Words::new("a b ").count(), 2);

    // Nothing to find.
    assert_eq!(Words::new("").next(), None);
    assert_eq!(Words::new(" \t\n").next(), None);

    // Not English: a multi-byte word and a multi-byte space (U+3000).
    let text = String::from("zürich\u{3000}bern");
    let words: Vec<_> = Words::new(&text).collect();
    assert_eq!(words, ["zürich", "bern"]);
    assert!(inside(&*words[0], &text));

    println!("all checks passed");
}
