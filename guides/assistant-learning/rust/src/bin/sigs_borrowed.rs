fn main() {
    let mut line = String::from("port=8080");
    let (key, _value) = line.split_once('=').unwrap();
    line.clear();
    println!("key: {key}");
}
