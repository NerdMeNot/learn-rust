use tinymoney::Currency;

fn symbol(currency: Currency) -> &'static str {
    match currency {
        Currency::Usd => "$",
        Currency::Eur => "EUR ",
        Currency::Jpy => "JPY ",
    }
}

fn main() {
    println!("{}", symbol(Currency::Jpy));
}
