fn main() {
    let word = "naïve";
    // ï takes bytes 2 and 3. Ending a slice at byte 3 cuts it in half.
    let start = &word[0..3];
    println!("{start}");
}
