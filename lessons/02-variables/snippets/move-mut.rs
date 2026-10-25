fn main() {
    let first = String::from("ab");
    let mut second = first; // the text moves into a variable that may change
    second.push('c');
    println!("second: {second}");
}
