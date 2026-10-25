// (1) Count the words in some text and report on them.
// (2)
#[derive(Debug)]
// (3)
struct Report {
    words: usize,
    longest: String,
}

// (4)
fn analyze(text: &str) -> Result<Report, String> {
    // (5)
    let mut longest = String::new();
    let mut words = 0;
    // (6)
    for word in text.split_whitespace() {
        words += 1;
        // (7)
        if word.len() > longest.len() {
            longest = word.to_string();
        }
    }
    if words == 0 {
        // (8)
        return Err(String::from("no words"));
    }
    // (9)
    Ok(Report { words, longest })
}

// (10)
fn main() -> Result<(), String> {
    // (11)
    let report = analyze("the quick brown fox")?;
    // (12)
    println!("{report:?}");
    // (13)
    let shout = |s: &str| s.to_uppercase();
    // (14)
    match report.words {
        0 => println!("empty"),
        1..=3 => println!("short"),
        n => println!("{n} words"),
    }
    println!("longest: {}", shout(&report.longest));
    Ok(())
}
