mod shop {
    mod rules {
        pub fn bulk_discount(qty: u32) -> u32 {
            if qty >= 10 { 20 } else { 0 }
        }
    }

    pub fn total(qty: u32) -> u32 {
        qty * 10 - qty * 10 * rules::bulk_discount(qty) / 100
    }
}

fn main() {
    println!("total: {}", shop::total(12));
    println!("discount: {}", shop::rules::bulk_discount(12));
}
