fn main() {
    let nickname = Some(String::from("Ada"));
    if let Some(n) = nickname {
        println!("hello, {n}");
    }
    println!("{:?}", nickname);
}
