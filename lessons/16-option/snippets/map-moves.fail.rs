fn main() {
    let nickname: Option<String> = Some(String::from("bo"));
    let length = nickname.map(|n| n.len());
    println!("length: {length:?}");
    println!("nickname: {nickname:?}");
}
