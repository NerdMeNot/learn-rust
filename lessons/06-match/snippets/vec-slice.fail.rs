fn main() {
    let v = vec![1, 2, 3];
    match v {
        [] => println!("empty"),
        [first, ..] => println!("starts with {first}"),
    }
}
