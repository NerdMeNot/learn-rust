fn main() {
    let text = "  tea  \n coffee \n  milk";
    let mut longest = "";
    for line in text.lines() {
        let trimmed = line.trim().to_string();
        if trimmed.len() > longest.len() {
            longest = &trimmed;
        }
    }
    println!("longest: {longest}");
}
