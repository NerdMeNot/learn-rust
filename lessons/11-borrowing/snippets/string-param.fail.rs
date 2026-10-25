fn is_greeting(text: &String) -> bool {
    text.starts_with("hello")
}

fn main() {
    println!(
        "is_greeting(\"hello there\"): {}",
        is_greeting("hello there")
    );
}
