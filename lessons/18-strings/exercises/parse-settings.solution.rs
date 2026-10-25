/// Turn "name = Ada; age=36 ;city= London" into pairs, trimming spaces around every
/// key and value. Skip pieces with no '=' and empty pieces.
fn parse_settings(text: &str) -> Vec<(String, String)> {
    // This uses iterator methods from the Iterators lesson; a for loop works too
    // (see parse_settings_with_a_loop below).
    text.split(';')
        .filter_map(|piece| piece.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

/// The same thing with a plain for loop.
#[allow(dead_code)]
fn parse_settings_with_a_loop(text: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for piece in text.split(';') {
        if let Some((key, value)) = piece.split_once('=') {
            pairs.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    pairs
}

fn main() {
    let got = parse_settings("name = Ada; age=36 ;city= London");
    assert_eq!(
        got,
        vec![
            ("name".to_string(), "Ada".to_string()),
            ("age".to_string(), "36".to_string()),
            ("city".to_string(), "London".to_string()),
        ]
    );
    assert_eq!(
        parse_settings(";;junk; a=b;"),
        vec![("a".to_string(), "b".to_string())]
    );
    assert!(parse_settings("").is_empty());
    println!("all checks passed");
}
