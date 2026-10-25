use tinymoney::{Amount, Currency, Error, Money, Rounding};

fn symbol(currency: Currency) -> &'static str {
    match currency {
        Currency::Usd => "$",
        Currency::Eur => "EUR ",
        // The crate marks `Currency` `#[non_exhaustive]`, so this arm is required.
        _ => "? ",
    }
}

fn describe(rounding: Rounding) -> &'static str {
    // `Rounding` is not `#[non_exhaustive]`, so naming every variant is enough.
    match rounding {
        Rounding::TowardZero => "drop the fraction",
        Rounding::HalfAwayFromZero => "round halves away from zero",
    }
}

fn main() -> Result<(), Error> {
    let price = Money::parse("19.99", Currency::Usd)?;
    let shipping = Money::new(Amount::from_minor(500), Currency::Usd);
    let total = price.checked_add(shipping)?;
    println!(
        "total: {total} ({}{})",
        symbol(total.currency()),
        total.amount().minor()
    );

    for rounding in [Rounding::TowardZero, Rounding::HalfAwayFromZero] {
        println!(
            "8% tax, {}: {}",
            describe(rounding),
            total.percent(8, rounding)?
        );
    }

    let euros = Money::parse("1", Currency::Eur)?;
    match total.checked_add(euros) {
        Ok(sum) => println!("sum: {sum}"),
        Err(err) => println!("refused: {err}"),
    }
    println!("{:?}", Money::parse("19.999", Currency::Usd));
    Ok(())
}
