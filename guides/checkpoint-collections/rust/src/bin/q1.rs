fn main() {
    let v = vec![1, 2, 3, 4];
    let mut evens_doubled = v
        .iter()
        .map(|x| {
            println!("map {x}");
            x * 2
        })
        .filter(|x| x % 4 == 0);
    println!("start");
    println!("{:?}", evens_doubled.next());
    println!("{:?}", evens_doubled.next());
}
