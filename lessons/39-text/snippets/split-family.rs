fn main() {
    // A separator at the end gives an empty last piece.
    println!("split(','): {:?}", "a,b,".split(',').collect::<Vec<_>>());
    println!(
        "split_terminator(','): {:?}",
        "a,b,".split_terminator(',').collect::<Vec<_>>()
    );

    // Nothing to split still gives one piece, but only for split(sep).
    println!("\"\".split(','): {:?}", "".split(',').collect::<Vec<_>>());
    println!(
        "\"\".split_whitespace(): {:?}",
        "".split_whitespace().collect::<Vec<_>>()
    );

    // Limits: the last piece keeps the rest.
    println!(
        "splitn(2, '='): {:?}",
        "k=v=w".splitn(2, '=').collect::<Vec<_>>()
    );
    println!(
        "rsplitn(2, '='): {:?}",
        "k=v=w".rsplitn(2, '=').collect::<Vec<_>>()
    );
    println!("rsplit_once('='): {:?}", "k=v=w".rsplit_once('='));

    // Keep the separator at the end of each piece.
    println!(
        "split_inclusive('\\n'): {:?}",
        "one\ntwo\n".split_inclusive('\n').collect::<Vec<_>>()
    );

    // split_whitespace uses Unicode White_Space: U+0085 splits; U+001C and U+FEFF do not.
    for sep in ['\u{85}', '\u{1c}', '\u{feff}'] {
        let text = format!("a{sep}b");
        println!(
            "split_whitespace around U+{:04X}: {:?}",
            sep as u32,
            text.split_whitespace().collect::<Vec<_>>()
        );
    }

    // lines is not split('\n').
    let text = "foo\r\nbar\n\nbaz\r";
    println!("lines: {:?}", text.lines().collect::<Vec<_>>());
    println!("split('\\n'): {:?}", text.split('\n').collect::<Vec<_>>());
    println!(
        "lines, lone \\r inside: {:?}",
        "a\rb\r\nc\n".lines().collect::<Vec<_>>()
    );
    println!(
        "lines on text ending in \\n: {:?}",
        "one\ntwo\n".lines().collect::<Vec<_>>()
    );
}
