fn main() {
    let s = String::from("héllo");
    let second = s.chars().nth(1);        // Some('é'): characters, not bytes
    let byte = s.as_bytes()[1];           // 195: the raw byte, if you want it
    println!("{second:?} {byte}");
}
