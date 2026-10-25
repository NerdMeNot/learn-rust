fn main() {
    let mut count = 0;
    let mut up = || count += 1;
    let mut down = || count -= 1;
    up();
    down();
    println!("{count}");
}
