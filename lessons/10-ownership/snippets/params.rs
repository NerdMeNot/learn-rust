// Only reads: borrow with &. The caller keeps the Vec.
fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

// Changes it in place: borrow mutably with &mut.
fn add_tax(prices: &mut Vec<u32>) {
    for p in prices.iter_mut() {
        *p += *p / 10;
    }
}

// Keeps it: take ownership. The caller hands the Vec over for good.
struct Basket {
    prices: Vec<u32>,
}

fn into_basket(prices: Vec<u32>) -> Basket {
    Basket { prices }
}

fn main() {
    let mut prices = vec![100, 250, 40];

    println!("total: {}", total(&prices));
    add_tax(&mut prices);
    println!("after tax: {prices:?}");

    let basket = into_basket(prices); // prices moves; we're done with it
    println!("basket: {:?}", basket.prices);
}
