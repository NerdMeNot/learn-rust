fn main() {
    let word = "héllo👋🏽";
    println!("graphemes: {}", word.graphemes(true).count());
}
