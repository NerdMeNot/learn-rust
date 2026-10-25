fn main() {
    let s = "héllo";
    println!("{} {}", s.len(), s.chars().count());
    println!("{:?}", s.get(..2));
    println!("{:?}", s.get(..3));
}
