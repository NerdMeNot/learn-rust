use std::fmt;
use std::str::FromStr;

use crate::Error;

/// An amount of money in whole cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Money(u64);

impl Money {
    /// Money from a number of cents.
    pub const fn from_cents(cents: u64) -> Money {
        Money(cents)
    }

    /// The amount in cents.
    pub const fn cents(self) -> u64 {
        self.0
    }

    /// `self + other`, or `None` if it does not fit.
    pub fn checked_add(self, other: Money) -> Option<Money> {
        self.0.checked_add(other.0).map(Money)
    }

    /// `self * quantity`, or `None` if it does not fit.
    pub fn checked_mul(self, quantity: u32) -> Option<Money> {
        self.0.checked_mul(u64::from(quantity)).map(Money)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, self.0 % 100)
    }
}

impl FromStr for Money {
    type Err = Error;

    /// Parses whole cents: `"1250"` is 12.50.
    fn from_str(s: &str) -> Result<Money, Error> {
        s.parse::<u64>().map(Money).map_err(|_| Error::InvalidPrice(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_dollars_and_cents() {
        assert_eq!(Money::from_cents(1250).to_string(), "12.50");
        assert_eq!(Money::from_cents(5).to_string(), "0.05");
    }

    #[test]
    fn arithmetic_reports_overflow() {
        let big = Money::from_cents(u64::MAX);
        assert_eq!(big.checked_add(Money::from_cents(1)), None);
        assert_eq!(big.checked_mul(2), None);
        assert_eq!(Money::from_cents(250).checked_mul(3), Some(Money::from_cents(750)));
    }

    #[test]
    fn parses_cents_only() {
        assert_eq!("1250".parse::<Money>(), Ok(Money::from_cents(1250)));
        assert!("12.50".parse::<Money>().is_err());
    }
}
