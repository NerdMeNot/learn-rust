fn main() {
    // split gives an iterator of &str pieces. collect turns it into a Vec.
    let parts: Vec<&str> = "a,b,,c".split(',').collect();
    println!("split(','): {parts:?}");

    // Whitespace of any kind and length, with no empty pieces.
    let words: Vec<&str> = "  the   quick\tbrown\nfox ".split_whitespace().collect();
    println!("split_whitespace: {words:?}");

    // One line at a time (handles \n and \r\n).
    for (i, line) in "first\nsecond\r\nthird".lines().enumerate() {
        println!("{i}: {line}");
    }

    // Split once, at the first match: perfect for key=value.
    if let Some((key, value)) = "port=8080".split_once('=') {
        println!("key={key} value={value}");
    }

    // At most n pieces.
    let v: Vec<&str> = "2026-10-24T10:30".splitn(2, 'T').collect();
    println!("splitn(2, 'T'): {v:?}");

    // Trimming returns a smaller view of the same text: nothing is copied.
    let raw = "   padded \n";
    println!("trim: {:?}", raw.trim());
    println!("trim_start: {:?}", raw.trim_start());
    println!("trim_end: {:?}", raw.trim_end());
    println!("trim_matches('x'): {:?}", "xxhixx".trim_matches('x'));

    // strip_prefix / strip_suffix: Some(rest) if it was there, None if not.
    println!("strip_prefix on v1.2.3: {:?}", "v1.2.3".strip_prefix('v'));
    println!("strip_prefix on 1.2.3: {:?}", "1.2.3".strip_prefix('v'));
}
