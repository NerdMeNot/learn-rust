fn main() {
    let word = "héllo👋🏽";
    // no `use` line: call the trait's method through its full path
    let n = unicode_segmentation::UnicodeSegmentation::graphemes(word, true).count();
    println!("graphemes: {n}");
}
