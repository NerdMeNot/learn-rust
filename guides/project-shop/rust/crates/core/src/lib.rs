//! The shop's domain: prices, SKUs, products, and the [`Catalog`] trait that
//! storage crates implement. This crate has no dependencies.

mod catalog;
mod error;
mod money;
mod product;

pub use catalog::{Catalog, order_total};
pub use error::Error;
pub use money::Money;
pub use product::{Product, Sku};
