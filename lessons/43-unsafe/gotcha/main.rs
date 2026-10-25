fn main() {
    let prices = [10, 20, 30];
    let p = prices.as_ptr();
    let second = *p.add(1);
    println!("second: {second}");
}
