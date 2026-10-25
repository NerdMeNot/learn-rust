fn main() {
    let mut count = 0;
    let mut inc = move || {
        count += 1;
        count
    };
    println!("closure's count: {}", inc());
    println!("closure's count: {}", inc());
    println!("outer count: {count}");
}
