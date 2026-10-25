fn main() {
    let mut evens = (1..=6)
        .filter(|n| {
            println!("check {n}");
            n % 2 == 0
        })
        .map(|n| n * 10);
    println!("built the iterator");
    println!("first: {:?}", evens.next());
    println!("count: {}", evens.count());
}
