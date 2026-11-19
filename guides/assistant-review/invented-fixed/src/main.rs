fn main() {
    let mut names = vec!["ana", "bob", "cy"];
    names.retain(|n| *n != "bob");
    println!("{names:?}");
}
