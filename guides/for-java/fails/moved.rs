fn main() {
    let a = vec![String::from("x")];
    let mut b = a;
    b.push(String::from("y"));
    println!("{a:?}");
}
