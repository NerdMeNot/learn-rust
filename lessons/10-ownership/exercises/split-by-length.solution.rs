/// Move each word into `short` (at most `max` characters) or `long`,
/// keeping their order. Don't clone: the caller has given you the words.
fn split_by_length(words: Vec<String>, max: usize) -> (Vec<String>, Vec<String>) {
    let mut short = Vec::new();
    let mut long = Vec::new();
    // Looping over `words` by value hands us each String to own.
    for word in words {
        if word.chars().count() <= max {
            short.push(word);
        } else {
            long.push(word);
        }
    }
    (short, long)
}

fn main() {
    let words = vec![
        String::from("ox"),
        String::from("crab"),
        String::from("café"),
        String::from("ferris"),
    ];
    let crab_text = words[1].as_ptr();
    let ferris_text = words[3].as_ptr();

    let (short, long) = split_by_length(words, 4);
    // "café" is 5 bytes but 4 characters, so it is short.
    assert_eq!(short, ["ox", "crab", "café"]);
    assert_eq!(long, ["ferris"]);
    // Same heap text as before: each String moved, none was copied.
    assert_eq!(
        short[1].as_ptr(),
        crab_text,
        "move the Strings, don't clone them"
    );
    assert_eq!(
        long[0].as_ptr(),
        ferris_text,
        "move the Strings, don't clone them"
    );

    let (short, long) = split_by_length(Vec::new(), 4);
    assert!(short.is_empty() && long.is_empty());

    let (short, long) = split_by_length(vec![String::new()], 0);
    assert_eq!(short, [""]);
    assert!(long.is_empty());
    println!("all checks passed");
}
