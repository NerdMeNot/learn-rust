/// Is the text a palindrome? Compare the chars for which `is_alphanumeric` is true,
/// ignoring case, spaces and punctuation. The empty text, and text with no letters,
/// count as one. Letters are precomposed: "é" is one char.
fn is_palindrome(text: &str) -> bool {
    let letters: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    letters.iter().eq(letters.iter().rev())
}

fn main() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(is_palindrome("12321"));
    assert!(is_palindrome(""));
    assert!(is_palindrome("?!"));
    // Not ASCII: É and é are the same letter, and the e is not dropped.
    assert!(is_palindrome("Été"));
    assert!(is_palindrome("été"));
    assert!(!is_palindrome("éaè"));
    assert!(!is_palindrome("Éa"));
    assert!(!is_palindrome("ab"));
    assert!(!is_palindrome("12 3"));
    // Same first and last letter, different middle: comparing only the ends is not enough.
    assert!(!is_palindrome("abca"));
    assert!(!is_palindrome("1231"));
    assert!(!is_palindrome("éaßé"));
    // Known limit, not a goal: "é" written as e plus U+0301 is two chars, and the accent
    // is not alphanumeric, so it is dropped. This text counts as a palindrome here.
    assert!(is_palindrome("e\u{301}ae\u{300}"));
    println!("all checks passed");
}
