fn main() {
    // trim removes Unicode White_Space, which includes a no-break space and
    // the wide space used in Japanese text. trim_ascii removes only ASCII ones.
    let padded = "\u{a0}\u{3000}hi\t\n";
    println!("trim:       {:?}", padded.trim());
    println!("trim_ascii: {:?}", padded.trim_ascii());
    // A control character is not White_Space, so trim keeps it.
    println!("trim on U+0001: {:?}", "\u{1}hi\u{1}".trim());

    // trim_*_matches removes the pattern again and again; strip_* removes it once.
    println!(
        "trim_start_matches('/'): {:?}",
        "///a/b".trim_start_matches('/')
    );
    println!(
        "strip_prefix(\"/\"):       {:?}",
        "///a/b".strip_prefix("/")
    );

    // A &str pattern is matched as a whole text: only the text "xa" goes.
    // To cut a SET of characters, pass an array of chars.
    println!(
        "trim_start_matches(\"xa\"):       {:?} {:?}",
        "xxabxx".trim_start_matches("xa"),
        "xaxab".trim_start_matches("xa")
    );
    println!(
        "trim_start_matches(['x', 'a']): {:?}",
        "xxabxx".trim_start_matches(['x', 'a'])
    );

    // A closure says which characters to cut away.
    println!(
        "trim_end_matches(digits): {:?}",
        "build-42".trim_end_matches(char::is_numeric)
    );
    println!(
        "trim_matches(not alphanumeric): {:?}",
        "(\"hello!\")".trim_matches(|c: char| !c.is_alphanumeric())
    );

    // strip_suffix tells you whether the suffix was there.
    for name in ["notes.md", "notes.txt"] {
        match name.strip_suffix(".md") {
            Some(stem) => println!("{name}: markdown, stem {stem:?}"),
            None => println!("{name}: not markdown"),
        }
    }
}
