fn main() {
    let t = (1, 2, 3, 4);
    let (first, rest @ ..) = t;
    println!("{first} {rest:?}");
}
