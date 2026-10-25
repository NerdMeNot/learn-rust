use std::fmt;
use std::str::FromStr;

use crate::Error;

/// A currency this crate knows about.
///
/// The enum is `#[non_exhaustive]`: a later version may add currencies without
/// a major version bump, so a `match` in another crate needs a `_` arm.
///
/// ```
/// use tinymoney::Currency;
///
/// assert_eq!("EUR".parse::<Currency>()?, Currency::Eur);
/// assert_eq!(Currency::Jpy.decimals(), 0);
/// # Ok::<(), tinymoney::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Currency {
    /// United States dollar.
    Usd,
    /// Euro.
    Eur,
    /// Japanese yen.
    Jpy,
    /// Pound sterling.
    Gbp,
}

impl Currency {
    /// The three-letter ISO 4217 code.
    pub fn code(self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Jpy => "JPY",
            Currency::Gbp => "GBP",
        }
    }

    /// How many decimal places the minor unit has: 2 for cents, 0 for yen.
    pub fn decimals(self) -> u32 {
        match self {
            Currency::Usd | Currency::Eur | Currency::Gbp => 2,
            Currency::Jpy => 0,
        }
    }

    /// The usual symbol, or `None` where this crate has none to offer.
    pub fn symbol(self) -> Option<&'static str> {
        match self {
            Currency::Usd => Some("$"),
            Currency::Gbp => Some("£"),
            _ => None,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(code: &str) -> Result<Currency, Error> {
        match code {
            "USD" => Ok(Currency::Usd),
            "EUR" => Ok(Currency::Eur),
            "JPY" => Ok(Currency::Jpy),
            "GBP" => Ok(Currency::Gbp),
            other => Err(Error::UnknownCurrency(other.to_string())),
        }
    }
}
