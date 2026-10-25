fn main() {
    let mut count = 0;
    let inc = || count += 1;
    inc();
    inc();
    println!("count={count}");
}
