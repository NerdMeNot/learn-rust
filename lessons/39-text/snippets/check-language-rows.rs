// Backs the Rust cells in the "Your text calls in Rust" tables; never shown.
use std::path::Path;

fn pieces<'a>(it: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    it.collect()
}

fn main() {
    // Go
    assert_eq!(pieces("a b  c".split_whitespace()), ["a", "b", "c"]);
    assert_eq!(pieces("k=v=w".splitn(2, '=')), ["k", "v=w"]);
    assert!("abc".contains("b"));
    assert_eq!("a-a-a".replacen("-", "+", 1), "a+a-a");
    assert_eq!("a-a-a".replace("-", "+"), "a+a+a");
    assert_eq!("abba-x".trim_start_matches(['a', 'b']), "-x");
    assert_eq!("///a".strip_prefix("/"), Some("//a"));
    assert!("3000000000".parse::<i64>().is_ok());
    assert!("3000000000".parse::<i32>().is_err());
    assert_eq!(Path::new(".bashrc").extension(), None);
    assert_eq!(Path::new("a.tar.gz").extension().unwrap(), "gz");

    // Java and Kotlin
    assert_eq!("+42".parse::<i32>().ok(), Some(42));
    assert_eq!(" 42".parse::<i32>().ok(), None);
    assert_eq!(pieces("a\r\nb\n".lines()), ["a", "b"]);
    assert_eq!("TITLE".to_lowercase(), "title");
    assert_eq!(pieces("a.b".split('.')), ["a", "b"]);
    assert_eq!(pieces("a,b,,".split(',')), ["a", "b", "", ""]);
    assert_eq!(pieces("a,b,c".splitn(2, ',')), ["a", "b,c"]);
    assert_eq!(pieces(" a  b".split_whitespace()), ["a", "b"]);
    assert!(!"É".eq_ignore_ascii_case("é"));
    assert!("٣".parse::<i32>().is_err());
    assert_eq!(
        "abc123".matches(char::is_numeric).collect::<Vec<_>>(),
        ["1", "2", "3"]
    );

    // C#
    let kept: Vec<&str> = "a,,b,".split(',').filter(|p| !p.is_empty()).collect();
    assert_eq!(kept, ["a", "b"]);
    assert!("1,5".parse::<f64>().is_err());

    // Python
    assert_eq!("xxabxx".trim_start_matches(['x', 'a']), "bxx");
    assert_eq!("xxabxx".trim_start_matches("xa"), "xxabxx");
    assert_eq!("a,b,c".rsplit_once(','), Some(("a,b", "c")));
    assert_eq!("a-a-a".matches("-").count(), 2);
    assert_eq!("a,b".find("z"), None);
    assert!("99999999999999999999".parse::<i64>().is_err());

    // C
    assert!("42abc".parse::<i64>().is_err());
    println!("all language rows hold");
}
