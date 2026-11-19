fn main() {
    let mut names = vec!["ana", "bob", "cy"];
    names.remove_item(&"bob");
    println!("{names:?}");
}
