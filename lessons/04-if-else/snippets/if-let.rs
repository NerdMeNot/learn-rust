fn main() {
    // `parse` returns a Result: Ok(number) or Err(reason). Errors are covered later.
    for text in ["42", "4x2", "99999999999"] {
        if let Ok(n) = text.parse::<i32>() {
            println!("{text:?} is the number {n}");
        } else {
            println!("{text:?} is not a number");
        }
    }

    // `else if let` chains several patterns
    let text = "7";
    if let Ok(n) = text.parse::<u8>() {
        println!("fits in a u8: {n}");
    } else if let Ok(n) = text.parse::<i64>() {
        println!("fits in an i64: {n}");
    } else {
        println!("not a whole number");
    }
}
