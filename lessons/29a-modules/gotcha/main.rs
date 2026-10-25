mod shop {
    // No `pub`.
    fn discount_percent(qty: u32) -> u32 {
        if qty >= 10 { 20 } else { 0 }
    }

    pub fn total(qty: u32) -> u32 {
        let full = qty * 10;
        full - full * discount_percent(qty) / 100
    }
}

fn main() {
    println!("total: {}", shop::total(12));
    println!("discount: {}%", shop::discount_percent(12));
}
