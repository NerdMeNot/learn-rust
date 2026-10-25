fn main() {
    // something that can fail returns Result
    match "42x".parse::<i32>() {
        Ok(n) => println!("got {n}"),
        Err(e) => println!("not a number: {e}"),
    }

    // something that might be missing returns Option
    let names = vec!["ana", "bo"];
    match names.get(5) {
        Some(n) => println!("found {n}"),
        None => println!("nobody at index 5"),
    }

    // a Result you ignore is a warning (see the build output above the run)
    "7".parse::<i32>();
}
