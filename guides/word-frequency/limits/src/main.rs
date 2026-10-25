mod words;

fn main() {
    let chars = [
        ('a', "LATIN SMALL LETTER A"),
        ('é', "LATIN SMALL LETTER E WITH ACUTE"),
        ('ß', "LATIN SMALL LETTER SHARP S"),
        ('你', "a Han character"),
        ('Ⅳ', "ROMAN NUMERAL FOUR"),
        ('٣', "ARABIC-INDIC DIGIT THREE"),
        ('5', "DIGIT FIVE"),
        ('-', "HYPHEN-MINUS"),
        ('\u{2019}', "RIGHT SINGLE QUOTATION MARK"),
        ('\u{301}', "COMBINING ACUTE ACCENT"),
        ('😀', "GRINNING FACE"),
    ];
    for (c, name) in chars {
        println!(
            "U+{:04X} is_alphabetic: {:<5} {name}",
            c as u32,
            c.is_alphabetic()
        );
    }

    println!();
    let texts = [
        "state-of-the-art",
        "3rd place",
        "don't",
        "don\u{2019}t",
        "caf\u{e9}",
        "cafe\u{301}",
        "你好世界",
        "हिन्दी",
        "ก็ได้",
    ];
    for text in texts {
        println!("{text:?} -> {:?}", words::words(text));
    }

    println!();
    println!("lowercase of Straße: {}", "Straße".to_lowercase());
    println!("lowercase of STRASSE: {}", "STRASSE".to_lowercase());
    println!("lowercase of ISTANBUL: {}", "ISTANBUL".to_lowercase());
    println!(
        "chars in the lowercase of İ (U+0130): {}",
        "İ".to_lowercase().chars().count()
    );
    println!("lowercase of ΟΔΥΣΣΕΥΣ: {}", "ΟΔΥΣΣΕΥΣ".to_lowercase());
}
