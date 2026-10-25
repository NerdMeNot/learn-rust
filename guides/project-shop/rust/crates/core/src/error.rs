use std::fmt;

use crate::Sku;

/// Everything that can go wrong in the shop.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A SKU that is not 3 to 12 characters of `A-Z`, `0-9` or `-`.
    InvalidSku(String),
    /// A price that is not a whole number of cents.
    InvalidPrice(String),
    /// A quantity that is not a whole number above zero.
    InvalidQuantity(String),
    /// No product has this SKU.
    NotFound(Sku),
    /// A total that does not fit in the price type.
    Overflow,
    /// The storage behind a [`Catalog`](crate::Catalog) failed.
    Store(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSku(s) => write!(f, "invalid SKU {s:?}: use 3 to 12 of A-Z, 0-9, -"),
            Error::InvalidPrice(s) => write!(f, "invalid price {s:?}: use whole cents"),
            Error::InvalidQuantity(s) => write!(f, "invalid quantity {s:?}: use a number above 0"),
            Error::NotFound(sku) => write!(f, "no product {sku}"),
            Error::Overflow => write!(f, "total is too large"),
            Error::Store(why) => write!(f, "storage failed: {why}"),
        }
    }
}

impl std::error::Error for Error {}
