use std::fmt;
use std::str::FromStr;

use crate::{Error, Money};

/// A stock-keeping unit: 3 to 12 characters of `A-Z`, `0-9` or `-`.
///
/// ```
/// use shop_core::Sku;
///
/// let sku: Sku = "TEA-01".parse()?;
/// assert_eq!(sku.as_str(), "TEA-01");
/// assert!("tea".parse::<Sku>().is_err());
/// # Ok::<(), shop_core::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sku(String);

impl Sku {
    /// The SKU as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Sku {
    type Err = Error;

    fn from_str(s: &str) -> Result<Sku, Error> {
        let ok_char = |c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-';
        if (3..=12).contains(&s.len()) && s.chars().all(ok_char) {
            Ok(Sku(s.to_string()))
        } else {
            Err(Error::InvalidSku(s.to_string()))
        }
    }
}

impl fmt::Display for Sku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Something the shop sells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    /// Its identifier.
    pub sku: Sku,
    /// What customers see.
    pub name: String,
    /// The price of one.
    pub price: Money,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sku, self.price, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sku_rules() {
        for good in ["TEA", "TEA-01", "ABCDEFGHIJKL"] {
            assert!(good.parse::<Sku>().is_ok(), "{good}");
        }
        for bad in ["", "TE", "tea", "TEA 01", "ABCDEFGHIJKLM"] {
            assert_eq!(bad.parse::<Sku>(), Err(Error::InvalidSku(bad.to_string())), "{bad:?}");
        }
    }
}
