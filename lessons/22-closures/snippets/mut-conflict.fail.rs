fn main() {
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    println!("count: {count}");
    inc();
}
