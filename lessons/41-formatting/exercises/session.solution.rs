use std::fmt;

/// A value that must never be printed.
struct Secret(String);

#[derive(Debug)]
struct Session {
    user: String,
    token: Secret,
    hits: u32,
}

// Debug must not show what is inside, not even its length.
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

fn main() {
    let s = Session {
        user: "ada".to_string(),
        token: Secret("s3cr3t".to_string()),
        hits: 3,
    };
    assert_eq!(
        (s.user.as_str(), s.token.0.as_str(), s.hits),
        ("ada", "s3cr3t", 3)
    );
    assert_eq!(
        format!("{:?}", s),
        r#"Session { user: "ada", token: <redacted>, hits: 3 }"#
    );
    assert_eq!(
        format!("{:#?}", s),
        "Session {\n    user: \"ada\",\n    token: <redacted>,\n    hits: 3,\n}"
    );
    assert!(!format!("{:?} {:#?}", s, s).contains(&s.token.0));

    // The text must not depend on the secret, even in other values.
    assert_eq!(format!("{:?}", Secret(String::new())), "<redacted>");
    let two = vec![Secret("a".to_string()), Secret("bb".to_string())];
    assert_eq!(format!("{:?}", two), "[<redacted>, <redacted>]");
    assert_eq!(format!("{:?}", Some(&s.token)), "Some(<redacted>)");
    println!("all checks passed");
}
