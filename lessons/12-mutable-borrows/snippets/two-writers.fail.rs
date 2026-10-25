fn main() {
    let mut n = 10;
    let a = &mut n;
    let b = &mut n;
    *a += 1;
    *b += 1;
    println!("n: {n}");
}
