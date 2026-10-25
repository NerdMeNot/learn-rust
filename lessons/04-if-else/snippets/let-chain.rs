fn main() {
    for text in ["42", "-3", "x"] {
        if let Ok(n) = text.parse::<i32>()
            && n > 0
        {
            println!("{text:?}: positive number {n}");
        } else {
            println!("{text:?}: not a positive number");
        }
    }
}
