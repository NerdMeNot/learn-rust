fn main() {
    for c in ['a', 'Z', '7', ' ', 'é'] {
        let class = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
            'a'..='z' | 'A'..='Z' => "letter",
            '0'..='9' => "digit",
            ' ' | '\t' | '\n' => "space",
            _ => "other",
        };
        println!("{c:?} -> {class}");
    }
}
