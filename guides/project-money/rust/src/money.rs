use std::fmt;

use crate::{Amount, Currency, Error, Rounding};

/// An [`Amount`] in one [`Currency`].
///
/// The fields are private, so every `Money` was made by [`Money::new`] or
/// [`Money::parse`] and carries its currency with it.
///
/// ```
/// use tinymoney::{Amount, Currency, Money};
///
/// let fee = Money::new(Amount::from_minor(250), Currency::Usd);
/// assert_eq!(fee.to_string(), "USD 2.50");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    amount: Amount,
    currency: Currency,
}

impl Money {
    /// Money from an amount and its currency.
    pub fn new(amount: Amount, currency: Currency) -> Money {
        Money { amount, currency }
    }

    /// The amount, in minor units.
    pub fn amount(self) -> Amount {
        self.amount
    }

    /// The currency.
    pub fn currency(self) -> Currency {
        self.currency
    }

    /// Reads a decimal amount such as `"19.99"`, `"5"` or `"-0.05"`.
    ///
    /// It fails with [`Error::TooManyDecimals`] instead of rounding.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidAmount`] if the text is not digits with at most one
    /// point, [`Error::TooManyDecimals`] if it has more decimal places than
    /// the currency, and [`Error::Overflow`] if the value, counted in minor
    /// units, does not fit in an `i64` (`i64::MIN` itself does).
    ///
    /// ```
    /// use tinymoney::{Currency, Error, Money};
    ///
    /// let cents = Money::parse("19.99", Currency::Usd)?;
    /// assert_eq!(cents.amount().minor(), 1999);
    ///
    /// let err = Money::parse("19.999", Currency::Usd).unwrap_err();
    /// assert_eq!(err.to_string(), "USD has 2 decimal places");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn parse(text: &str, currency: Currency) -> Result<Money, Error> {
        let invalid = || Error::InvalidAmount(text.to_string());
        let (negative, body) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (whole, fraction) = body.split_once('.').unwrap_or((body, ""));
        let all_digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
        if whole.is_empty()
            || !all_digits(whole)
            || !all_digits(fraction)
            || (body.contains('.') && fraction.is_empty())
        {
            return Err(invalid());
        }
        let decimals = currency.decimals();
        if fraction.len() > decimals as usize {
            return Err(Error::TooManyDecimals {
                currency,
                max: decimals,
            });
        }
        // Work with the magnitude as a u64, so that i64::MIN (whose magnitude is
        // one more than i64::MAX) can be parsed too.
        let whole: u64 = whole.parse().map_err(|_| Error::Overflow)?;
        let fraction: u64 = if fraction.is_empty() {
            0
        } else {
            let digits: u64 = fraction.parse().map_err(|_| invalid())?;
            digits * 10_u64.pow(decimals - fraction.len() as u32)
        };
        let magnitude = whole
            .checked_mul(10_u64.pow(decimals))
            .and_then(|w| w.checked_add(fraction))
            .ok_or(Error::Overflow)?;
        let signed = if negative {
            -i128::from(magnitude)
        } else {
            i128::from(magnitude)
        };
        let minor = i64::try_from(signed).map_err(|_| Error::Overflow)?;
        Ok(Money::new(Amount::from_minor(minor), currency))
    }

    /// Adds two amounts of the same currency.
    ///
    /// # Errors
    ///
    /// [`Error::CurrencyMismatch`] if the currencies differ and
    /// [`Error::Overflow`] if the sum does not fit.
    ///
    /// ```
    /// use tinymoney::{Currency, Error, Money};
    ///
    /// let usd = Money::parse("1", Currency::Usd)?;
    /// let eur = Money::parse("1", Currency::Eur)?;
    /// assert_eq!(usd.checked_add(eur).unwrap_err().to_string(), "cannot combine USD and EUR");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn checked_add(self, other: Money) -> Result<Money, Error> {
        if self.currency != other.currency {
            return Err(Error::CurrencyMismatch {
                left: self.currency,
                right: other.currency,
            });
        }
        let amount = self
            .amount
            .checked_add(other.amount)
            .ok_or(Error::Overflow)?;
        Ok(Money::new(amount, self.currency))
    }

    /// `percent` per cent of this amount, rounded as `rounding` says.
    ///
    /// # Errors
    ///
    /// [`Error::Overflow`] if the result does not fit.
    ///
    /// ```
    /// use tinymoney::{Currency, Error, Money, Rounding};
    ///
    /// let total = Money::parse("24.99", Currency::Usd)?;
    /// assert_eq!(total.percent(8, Rounding::TowardZero)?.to_string(), "USD 1.99");
    /// assert_eq!(total.percent(8, Rounding::HalfAwayFromZero)?.to_string(), "USD 2.00");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn percent(self, percent: i64, rounding: Rounding) -> Result<Money, Error> {
        let scaled = i128::from(self.amount.minor()) * i128::from(percent);
        let minor = i64::try_from(rounding.divide(scaled, 100)).map_err(|_| Error::Overflow)?;
        Ok(Money::new(Amount::from_minor(minor), self.currency))
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minor = self.amount.minor();
        let sign = if minor < 0 { "-" } else { "" };
        let magnitude = minor.unsigned_abs();
        let decimals = self.currency.decimals();
        if decimals == 0 {
            return write!(f, "{} {sign}{magnitude}", self.currency);
        }
        let scale = 10_u64.pow(decimals);
        let (whole, fraction) = (magnitude / scale, magnitude % scale);
        write!(
            f,
            "{} {sign}{whole}.{fraction:0width$}",
            self.currency,
            width = decimals as usize
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_table() {
        let cases = [
            ("19.99", Currency::Usd, Ok(1999)),
            ("5", Currency::Usd, Ok(500)),
            ("5.1", Currency::Usd, Ok(510)),
            ("-0.05", Currency::Usd, Ok(-5)),
            ("500", Currency::Jpy, Ok(500)),
            (
                "19.999",
                Currency::Usd,
                Err(Error::TooManyDecimals {
                    currency: Currency::Usd,
                    max: 2,
                }),
            ),
            (
                "5.5",
                Currency::Jpy,
                Err(Error::TooManyDecimals {
                    currency: Currency::Jpy,
                    max: 0,
                }),
            ),
            ("", Currency::Usd, Err(Error::InvalidAmount(String::new()))),
            ("1.", Currency::Usd, Err(Error::InvalidAmount("1.".into()))),
            (
                "1e3",
                Currency::Usd,
                Err(Error::InvalidAmount("1e3".into())),
            ),
            ("99999999999999999999", Currency::Usd, Err(Error::Overflow)),
        ];
        for (text, currency, expected) in cases {
            let got = Money::parse(text, currency).map(|m| m.amount().minor());
            assert_eq!(got, expected, "parsing {text:?}");
        }
    }

    #[test]
    fn parse_reaches_both_ends_of_i64() {
        let minor = |text, currency| Money::parse(text, currency).map(|m| m.amount().minor());
        assert_eq!(minor("-92233720368547758.08", Currency::Usd), Ok(i64::MIN));
        assert_eq!(minor("92233720368547758.07", Currency::Usd), Ok(i64::MAX));
        assert_eq!(minor("-9223372036854775808", Currency::Jpy), Ok(i64::MIN));
        assert_eq!(minor("9223372036854775807", Currency::Jpy), Ok(i64::MAX));
        // One past either end is still an error.
        assert_eq!(minor("-92233720368547758.09", Currency::Usd), Err(Error::Overflow));
        assert_eq!(minor("92233720368547758.08", Currency::Usd), Err(Error::Overflow));
        assert_eq!(minor("-9223372036854775809", Currency::Jpy), Err(Error::Overflow));
        assert_eq!(minor("9223372036854775808", Currency::Jpy), Err(Error::Overflow));
    }

    #[test]
    fn display_pads_the_fraction() {
        let show = |minor, currency| Money::new(Amount::from_minor(minor), currency).to_string();
        assert_eq!(show(5, Currency::Usd), "USD 0.05");
        assert_eq!(show(-5, Currency::Usd), "USD -0.05");
        assert_eq!(show(500, Currency::Jpy), "JPY 500");
        assert_eq!(show(i64::MIN, Currency::Jpy), "JPY -9223372036854775808");
    }

    #[test]
    fn adding_overflow_is_an_error() {
        let max = Money::new(Amount::from_minor(i64::MAX), Currency::Eur);
        let one = Money::new(Amount::from_minor(1), Currency::Eur);
        assert_eq!(max.checked_add(one), Err(Error::Overflow));
    }

    #[test]
    fn percent_rounds_negative_amounts_away_from_zero() {
        let debt = Money::new(Amount::from_minor(-250), Currency::Usd);
        let half = debt.percent(1, Rounding::HalfAwayFromZero).unwrap();
        assert_eq!(half.amount().minor(), -3);
    }
}
