fn main() {
    let scores = [1.0, 2.26, 3.0];
    println!("{:?}", scores);
    println!("{:.1?}", scores); // Debug on a slice passes the spec to each item
    let words: Vec<String> = scores.iter().map(|s| format!("{s:.1}")).collect();
    println!("{}", words.join(", "));
}
