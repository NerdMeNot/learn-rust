fn main() {
    let doubled = (1..4).map(|x| x * 2).collect();
    println!("{}", doubled.len());
}
