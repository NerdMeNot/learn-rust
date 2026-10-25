fn main() {
    // Case folding is what Unicode defines for comparing text without case.
    // std has a method for it, but on Rust 1.99.1 it is nightly-only.
    let a = "Straße".to_casefold_unnormalized();
    let b = "STRASSE".to_casefold_unnormalized();
    println!("equal ignoring case: {}", a == b);
}
