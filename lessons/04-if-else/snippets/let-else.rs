fn double(text: &str) -> i32 {
    let Ok(n) = text.parse::<i32>() else {
        println!("  cannot parse {text:?}, using 0");
        return 0;
    };
    // `n` is in scope for the rest of the function, not inside a block
    n * 2
}

fn main() {
    println!("double(\"21\"): {}", double("21"));
    println!("double(\"abc\"): {}", double("abc"));
}
