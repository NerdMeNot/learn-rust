mod shop {
    const PRICE: u32 = 10;

    // No `pub`: only code inside `shop` can call this.
    fn discount_percent(qty: u32) -> u32 {
        if qty >= 10 { 20 } else { 0 }
    }

    pub fn total(qty: u32) -> u32 {
        let full = qty * PRICE;
        full - full * discount_percent(qty) / 100
    }
}

fn main() {
    println!("3 items: {}", shop::total(3));
    println!("12 items: {}", shop::total(12));
}
