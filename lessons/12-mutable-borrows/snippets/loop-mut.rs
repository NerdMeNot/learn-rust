fn main() {
    let mut prices = vec![100, 250, 80];

    for p in &mut prices {
        *p += 5;
    }
    println!("after for p in &mut prices: {prices:?}");

    for i in 0..prices.len() {
        prices[i] *= 2;
    }
    println!("after prices[i] *= 2: {prices:?}");

    prices[1] -= 50;
    println!("after prices[1] -= 50: {prices:?}");
}
