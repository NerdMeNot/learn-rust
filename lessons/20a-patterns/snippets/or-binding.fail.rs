fn main() {
    let pair = (Some(1), Some(2));
    match pair {
        // `a` is bound on the left of the `|` but not on the right
        (Some(a), _) | (_, None) => println!("matched"),
        _ => println!("other"),
    }
}
