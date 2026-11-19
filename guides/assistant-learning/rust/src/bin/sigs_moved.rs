fn main() {
    let word = String::from("hi");
    let bytes = word.into_bytes();
    println!("{bytes:?} from {word}");
}
