fn main() {
    // `n` exists only in the first block (unlike Go, not in the `else`)
    if let Ok(n) = "42".parse::<i32>() {
        println!("number: {n}");
    } else {
        println!("not a number");
    }

    // for an init statement that is not a pattern, a block gives the same scope
    {
        let len = "hello".len();
        if len > 3 {
            println!("long: {len}");
        }
    }
}
