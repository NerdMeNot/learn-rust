fn main() {
    let mut x = 1;
    std::mem::swap(&mut x, &mut x);
    println!("x: {x}");
}
