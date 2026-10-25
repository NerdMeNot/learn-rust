/// A document section: a title line, then the body. It borrows both from the document.
struct Section<'a> {
    title: &'a str,
    body: &'a str,
}

impl<'a> Section<'a> {
    /// "Title\nthe rest" -> the title is the first line, the body is everything after
    /// the first newline. No newline: no section.
    fn parse(document: &'a str) -> Option<Section<'a>> {
        let (title, body) = document.split_once('\n')?;
        Some(Section { title, body })
    }

    /// The first whitespace-separated word of the body, or "" when there is none.
    /// It borrows from the document (`'a`), not from the `Section` that found it.
    fn first_word(&self) -> &'a str {
        self.body.split_whitespace().next().unwrap_or("")
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
