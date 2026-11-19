fn main() {
    // push_str(&mut self, string: &str): changes the String, borrows the text.
    let mut greeting = String::from("hello");
    let name = String::from("ferris");
    greeting.push_str(", ");
    greeting.push_str(&name); // a &String is accepted where &str is asked for
    println!("push_str: {greeting}, and name is still {name}");

    // split_once(&self, ..) -> Option<(&str, &str)>: both halves borrow `line`.
    let line = String::from("port=8080");
    match line.split_once('=') {
        Some((key, value)) => println!("split_once: key {key:?}, value {value:?}"),
        None => println!("split_once: no '='"),
    }

    // parse::<F>(&self) -> Result<F, F::Err>: the text is only read; the result may fail.
    let good: Result<u16, _> = "8080".parse();
    let bad: Result<u16, _> = "eighty".parse();
    println!("parse: {good:?} {bad:?}");

    // into_bytes(self) -> Vec<u8>: takes the String; nothing is copied.
    let word = String::from("hi");
    let bytes = word.into_bytes();
    println!("into_bytes: {bytes:?}");
}
