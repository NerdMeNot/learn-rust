fn main() {
    let word = "noe\u{301}l"; // n, o, e, a combining acute accent, l
    let reversed: String = word.chars().rev().collect();

    println!("word:     {word}  {word:?}");
    println!("reversed: {reversed}  {reversed:?}");
    println!("chars: {}, bytes: {}", word.chars().count(), word.len());
}
