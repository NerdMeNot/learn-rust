fn main() {
    let line = "mode";
    match line.find('=') {
        Some(i) => println!("value: {}", &line[i + 1..]),
        None => println!("no '=' in {line:?}"),
    }
}
