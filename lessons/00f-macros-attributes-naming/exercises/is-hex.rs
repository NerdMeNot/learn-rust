/// True if `c` is one of the characters 0-9, a-f or A-F.
/// Hint: `matches!(value, pattern)` is true when the value fits the pattern.
/// A pattern can be a range such as `'a'..='f'`, and `|` means "or".
fn is_hex_digit(c: char) -> bool {
    todo!("matches! with three ranges joined by |")
}

fn main() {
    for c in ['0', '9', 'a', 'f', 'F'] {
        assert!(is_hex_digit(c), "{c} should be a hex digit");
    }
    for c in ['g', 'G', ' ', '/'] {
        assert!(!is_hex_digit(c), "{c} should not be a hex digit");
    }
    // The case to watch: look-alike digits and letters from other alphabets.
    assert!(!is_hex_digit('٣'), "an Arabic-Indic digit is not 0-9");
    assert!(!is_hex_digit('ａ'), "a full-width letter is not a-f");
    println!("all checks passed");
}
