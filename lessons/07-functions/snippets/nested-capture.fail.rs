fn total_with_tax(price: u32) -> u32 {
    let rate = 20;
    fn tax(p: u32) -> u32 {
        p * rate / 100
    }
    println!("rate: {rate}");
    price + tax(price)
}

fn main() {
    println!("total: {}", total_with_tax(50));
}
