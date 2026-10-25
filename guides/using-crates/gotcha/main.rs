use unicode-segmentation::UnicodeSegmentation;

fn main() {
    let word = "héllo👋🏽";
    println!("bytes: {}", word.len());
    println!("chars: {}", word.chars().count());
    // what a person would call "letters": the hand and its skin tone count as one
    println!("graphemes: {}", word.graphemes(true).count());
}
