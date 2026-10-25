/// Build the text "== NAME ==" with `name` in upper case (all Unicode letters, not just ASCII).
/// The text is made inside the function, so the caller has to own it: fix the return type.
fn banner(name: &str) -> &str {
    todo!("build the text with format!, then change -> &str so the caller owns it")
}

/// True when `T` is `String` (a `&str`, even a leaked one, is not).
fn is_string<T>(_: &T) -> bool {
    std::any::type_name::<T>() == "alloc::string::String"
}

fn main() {
    let b = banner("rust");
    assert_eq!(b, "== RUST ==");
    assert!(is_string(&b), "return an owned String");

    // Not English: to_ascii_uppercase would leave the é alone.
    assert_eq!(banner("café"), "== CAFÉ ==");
    assert_eq!(banner(""), "==  ==");

    println!("all checks passed");
}
