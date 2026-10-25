//! Money as whole minor units (cents, yen) with a currency attached.
//!
//! Amounts are integers, so adding them never rounds. Two currencies cannot be
//! added by accident: that is an [`Error`], not a wrong number.
//!
//! ```
//! use tinymoney::{Currency, Money};
//!
//! let price = Money::parse("19.99", Currency::Usd)?;
//! let shipping = Money::parse("5", Currency::Usd)?;
//! assert_eq!(price.checked_add(shipping)?.to_string(), "USD 24.99");
//! # Ok::<(), tinymoney::Error>(())
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod amount;
mod currency;
mod error;
mod money;
mod rounding;

pub use amount::Amount;
pub use currency::Currency;
pub use error::Error;
pub use money::Money;
pub use rounding::Rounding;
