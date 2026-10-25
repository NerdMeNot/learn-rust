/// Shipping in cents. Up to 1 kg costs 500, over 1 kg and up to 5 kg costs 900,
/// over 5 kg costs 1500. Express delivery doubles the price.
fn shipping_cents(weight_kg: u32, express: bool) -> u32 {
    let base = if weight_kg <= 1 {
        500
    } else if weight_kg <= 5 {
        900
    } else {
        1500
    };
    if express { base * 2 } else { base }
}

fn main() {
    assert_eq!(shipping_cents(0, false), 500);
    assert_eq!(shipping_cents(1, false), 500);
    assert_eq!(shipping_cents(2, false), 900);
    assert_eq!(shipping_cents(5, false), 900);
    assert_eq!(shipping_cents(6, false), 1500);
    assert_eq!(shipping_cents(5, true), 1800);
    assert_eq!(shipping_cents(1, true), 1000);
    println!("all checks passed");
}
