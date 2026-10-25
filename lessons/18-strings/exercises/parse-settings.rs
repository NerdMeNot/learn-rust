/// Turn "name = Ada; age=36 ;city= London" into pairs, trimming spaces around every
/// key and value. Skip pieces with no '=' and empty pieces.
fn parse_settings(text: &str) -> Vec<(String, String)> {
    todo!("split on ';', then split_once('=') and trim")
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
