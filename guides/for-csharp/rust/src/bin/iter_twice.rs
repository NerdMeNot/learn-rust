fn main() {
    let evens = (1..=6).filter(|n| n % 2 == 0).map(|n| n * 10);
    println!("count: {}", evens.count());
    println!("count: {}", evens.count());
}
