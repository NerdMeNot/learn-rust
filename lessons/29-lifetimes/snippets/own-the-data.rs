use std::mem::size_of;

// Borrowing: nothing is copied, but the lifetime spreads to every type that holds it.
struct Word<'a> {
    text: &'a str,
}
struct Line<'a> {
    words: Vec<Word<'a>>,
}

// Owning: the text is copied once, and no lifetime appears anywhere.
struct OwnedWord {
    text: String,
}
struct OwnedLine {
    words: Vec<OwnedWord>,
}

fn borrowed_line(raw: &str) -> Line<'_> {
    Line {
        words: raw.split(' ').map(|text| Word { text }).collect(),
    }
}

fn owned_line(raw: &str) -> OwnedLine {
    OwnedLine {
        words: raw
            .split(' ')
            .map(|text| OwnedWord {
                text: text.to_string(),
            })
            .collect(),
    }
}

fn main() {
    let owned;
    {
        let raw = String::from("own the data");
        let borrowed = borrowed_line(&raw);
        println!("borrowed, last word: {}", borrowed.words[2].text);
        owned = owned_line(&raw);
    } // `raw` is gone: `borrowed` could not be used here, `owned` can
    println!("owned, last word: {}", owned.words[2].text);
    println!("size_of::<Word>(): {}", size_of::<Word>());
    println!("size_of::<OwnedWord>(): {}", size_of::<OwnedWord>());
}
