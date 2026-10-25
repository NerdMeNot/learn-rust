fn main() {
    let text = "5";
    if let Ok(n) = text.parse::<i32>() || text.is_empty() {
        println!("{n}");
    }
}
