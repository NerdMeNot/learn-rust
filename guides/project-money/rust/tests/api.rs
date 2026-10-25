//! Tests that use the library only through its public API, as another crate would.

use tinymoney::{Amount, Currency, Error, Money, Rounding};

#[test]
fn a_shopping_basket_adds_up() -> Result<(), Error> {
    let prices = ["19.99", "5", "0.01"];
    let mut total = Money::new(Amount::ZERO, Currency::Usd);
    for text in prices {
        total = total.checked_add(Money::parse(text, Currency::Usd)?)?;
    }
    assert_eq!(total.to_string(), "USD 25.00");
    Ok(())
}

#[test]
fn display_and_parse_agree() -> Result<(), Error> {
    for text in ["0.00", "12.34", "-7.05"] {
        let money = Money::parse(text, Currency::Eur)?;
        assert_eq!(money.to_string(), format!("EUR {text}"));
    }
    Ok(())
}

#[test]
fn errors_say_what_went_wrong() {
    let err = "ZZZ".parse::<Currency>().unwrap_err();
    assert_eq!(err, Error::UnknownCurrency("ZZZ".to_string()));
    assert_eq!(err.to_string(), "unknown currency code \"ZZZ\"");
}

#[test]
fn rounding_modes_differ_on_a_half() -> Result<(), Error> {
    let fee = Money::parse("0.05", Currency::Usd)?;
    let down = fee.percent(50, Rounding::TowardZero)?;
    let up = fee.percent(50, Rounding::HalfAwayFromZero)?;
    assert_eq!((down.amount().minor(), up.amount().minor()), (2, 3));
    Ok(())
}

#[test]
fn rounding_modes_differ_above_a_half_and_agree_below_it() -> Result<(), Error> {
    let total = Money::parse("24.99", Currency::Usd)?;
    // 24.99 x 8% = 1.9992: 199.92 cents, 0.92 of a cent is more than a half.
    assert_eq!(total.percent(8, Rounding::TowardZero)?.to_string(), "USD 1.99");
    assert_eq!(total.percent(8, Rounding::HalfAwayFromZero)?.to_string(), "USD 2.00");
    // 24.99 x 60% = 14.994: 1499.4 cents, 0.4 of a cent is less than a half.
    assert_eq!(total.percent(60, Rounding::TowardZero)?.to_string(), "USD 14.99");
    assert_eq!(total.percent(60, Rounding::HalfAwayFromZero)?.to_string(), "USD 14.99");
    Ok(())
}

#[test]
fn negative_halves_go_away_from_zero() -> Result<(), Error> {
    let debt = Money::parse("-2.50", Currency::Usd)?;
    assert_eq!(debt.percent(1, Rounding::TowardZero)?.to_string(), "USD -0.02");
    assert_eq!(debt.percent(1, Rounding::HalfAwayFromZero)?.to_string(), "USD -0.03");
    Ok(())
}
