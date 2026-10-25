fn main() {
    let mut n = 10;
    let reader = &n;
    let writer = &mut n;
    *writer += 1;
    println!("reader sees {reader}");
}
