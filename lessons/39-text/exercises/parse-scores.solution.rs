/// "Ana: 90, Bo: 85" -> [("Ana", 90), ("Bo", 85)].
/// Spaces around names and numbers are ignored, and empty items (a trailing
/// comma, two commas in a row) are skipped. The first bad item stops the parse:
/// - no ':' in it: Err(format!("missing ':' in {item:?}")), item already trimmed
/// - a score that is not a u32: Err(format!("{name}: {e}")) with the parse error's message
fn parse_scores(text: &str) -> Result<Vec<(String, u32)>, String> {
    let mut scores = Vec::new();
    for item in text.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        let (name, score) = item
            .split_once(':')
            .ok_or_else(|| format!("missing ':' in {item:?}"))?;
        let name = name.trim();
        let score = score
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("{name}: {e}"))?;
        scores.push((name.to_string(), score));
    }
    Ok(scores)
}

fn main() {
    let ok = parse_scores("Ana: 90, Bo:85 ,Zoë : 7,").unwrap();
    assert_eq!(
        ok,
        [
            ("Ana".to_string(), 90),
            ("Bo".to_string(), 85),
            ("Zoë".to_string(), 7)
        ]
    );
    assert_eq!(parse_scores(""), Ok(vec![]));
    assert_eq!(parse_scores(" , ,"), Ok(vec![]));

    assert_eq!(
        parse_scores("Ana: 1, Bo: x"),
        Err("Bo: invalid digit found in string".to_string())
    );
    assert_eq!(
        parse_scores("Cy: -5"),
        Err("Cy: invalid digit found in string".to_string())
    );
    assert_eq!(
        parse_scores("Di: 99999999999"),
        Err("Di: number too large to fit in target type".to_string())
    );
    assert_eq!(
        parse_scores("Ed: "),
        Err("Ed: cannot parse integer from empty string".to_string())
    );
    assert_eq!(
        parse_scores("Fay 5, Bo: x"),
        Err("missing ':' in \"Fay 5\"".to_string())
    );
    // The item in the message is already trimmed, however it was padded.
    assert_eq!(
        parse_scores("Ana: 1,  Fay 5 ,"),
        Err("missing ':' in \"Fay 5\"".to_string())
    );
    println!("all checks passed");
}
