fn main() {
    let nick: Option<String> = Some(String::from("ana"));
    if let Some(n) = nick {
        println!("hello, {n}");
    }
    println!("nick is {nick:?}");
}
