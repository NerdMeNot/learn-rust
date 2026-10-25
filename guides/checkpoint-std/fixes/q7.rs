fn main() {
    let word = "héllo";
    let start: String = word.chars().take(2).collect();
    println!("{start}");
}
