// `discounts` isn't `pub` in pricing.rs, so only pricing can call this.
pub fn bulk(quantity: u32, gross: u64) -> u64 {
    if quantity >= 3 { gross / 10 } else { 0 }
}
