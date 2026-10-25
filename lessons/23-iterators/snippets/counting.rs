fn main() {
    let prices = [3, 4, 5];

    // sum and product can produce several types, so name the one you want.
    let total: i32 = prices.iter().sum();
    let product = prices.iter().product::<i32>();
    println!("sum: {total}, product: {product}");

    // An empty sum is 0 and an empty product is 1, not None.
    let none: [i32; 0] = [];
    println!("empty sum: {}", none.iter().sum::<i32>());
    println!("empty product: {}", none.iter().product::<i32>());

    // Floats work too.
    let avg = [1.5, 2.5].iter().sum::<f64>() / 2.0;
    println!("average: {avg}");

    // count walks the whole iterator.
    println!(
        "count of evens: {}",
        (1..=10).filter(|n| n % 2 == 0).count()
    );
}
