fn main() {
    // parse returns a Result: Ok(the number) or Err(what went wrong).
    let good = "42".parse::<u32>();
    let bad = "4x2".parse::<u32>();
    println!("good: {good:?}");
    println!("bad: {bad:?}");
    println!("good.is_ok(): {}", good.is_ok());
    println!("bad.is_err(): {}", bad.is_err());

    // match opens a Result. The compiler checks that both cases are handled.
    for text in ["42", "4x2", "", "-7", "99999999999"] {
        match text.parse::<u32>() {
            Ok(n) => println!("{text:?} -> number {n}"),
            Err(e) => println!("{text:?} -> error: {e}"),
        }
    }
}
