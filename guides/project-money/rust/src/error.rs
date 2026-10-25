use crate::Currency;

/// What can go wrong when making or combining money.
///
/// `#[non_exhaustive]`, like [`Currency`]: new kinds of failure can be added in
/// a minor version.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Two amounts in different currencies were combined.
    #[error("cannot combine {left} and {right}")]
    CurrencyMismatch {
        /// The currency of the left-hand side.
        left: Currency,
        /// The currency of the right-hand side.
        right: Currency,
    },
    /// The result does not fit in 64 bits of minor units.
    #[error("the amount does not fit in an i64 of minor units")]
    Overflow,
    /// The text is not a decimal amount.
    #[error("{0:?} is not an amount")]
    InvalidAmount(String),
    /// More decimal places than the currency has.
    #[error("{currency} has {max} decimal places")]
    TooManyDecimals {
        /// The currency the amount was parsed for.
        currency: Currency,
        /// How many decimal places it allows.
        max: u32,
    },
    /// A currency code this crate does not know.
    #[error("unknown currency code {0:?}")]
    UnknownCurrency(String),
}
