use tinymoney::{Amount, Currency, Money};

fn main() {
    // Tuple-struct syntax would skip `from_minor`, the one place the meaning is spelled out.
    let fee = Money::new(Amount(250), Currency::Usd);
    println!("{fee}");
}
