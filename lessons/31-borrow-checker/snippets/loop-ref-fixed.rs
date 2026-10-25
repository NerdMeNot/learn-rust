fn main() {
    let text = "  tea  \n coffee \n  milk";

    // Borrow from `text`, which lives for the whole function: no String needed.
    let mut longest = "";
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.len() > longest.len() {
            longest = trimmed;
        }
    }
    println!("borrowed: {longest}");

    // If each line really is built in the loop, move the String out instead.
    let mut longest = String::new();
    for line in text.lines() {
        let upper = line.trim().to_uppercase();
        if upper.len() > longest.len() {
            longest = upper;
        }
    }
    println!("owned: {longest}");
}
