fn main() {
    let scores = vec![40, 75, 90];
    for score in &scores {
        if score > 60 {
            println!("{score} passes");
        }
    }
}
