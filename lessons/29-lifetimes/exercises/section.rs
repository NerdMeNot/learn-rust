/// A document section: a title line, then the body.
///
/// The starter owns its text, so it compiles. Make it borrow from the document instead:
/// give `Section` a lifetime, and have `first_word` return a `&str` that points into the
/// document and does not borrow from the `Section` itself.
struct Section {
    title: String,
    body: String,
}

impl Section {
    /// "Title\nthe rest" -> the title is the first line, the body is everything after
    /// the first newline. No newline: no section.
    fn parse(document: &str) -> Option<Section> {
        todo!("split_once('\\n')")
    }

    /// The first whitespace-separated word of the body, or "" when there is none.
    fn first_word(&self) -> String {
        todo!("split_whitespace")
    }
}

/// Do the bytes of `piece` lie inside the bytes of `text`? A copy would not.
fn inside(piece: &str, text: &str) -> bool {
    let start = text.as_ptr() as usize;
    let at = piece.as_ptr() as usize;
    at >= start && at + piece.len() <= start + text.len()
}

fn main() {
    let document = String::from("Lifetimes\nare regions of the program\nnot numbers");
    let section = Section::parse(&document).expect("has a title line");
    assert_eq!(section.title, "Lifetimes");
    assert_eq!(section.body, "are regions of the program\nnot numbers");
    assert!(inside(&*section.title, &document));
    assert!(inside(&*section.body, &document));

    // The word points into the document, so it can outlive the section.
    let word = section.first_word();
    drop(section);
    assert_eq!(word, "are");
    assert!(inside(&*word, &document));

    // No newline, no section.
    assert!(Section::parse("just a title").is_none());

    // An empty body has no first word.
    let document = String::from("Title\n");
    let section = Section::parse(&document).unwrap();
    assert_eq!(section.first_word(), "");

    // Not English, and nothing is trimmed: title and body are exactly the text.
    let document = String::from("Zürich\n  über den Wolken");
    let section = Section::parse(&document).unwrap();
    assert_eq!(section.title, "Zürich");
    assert_eq!(section.body, "  über den Wolken");
    assert_eq!(section.first_word(), "über");
    let section = Section::parse(" padded \nx ").unwrap();
    assert_eq!(section.title, " padded ");
    assert_eq!(section.body, "x ");

    println!("all checks passed");
}
