//! Implementations of [`shop_core::Catalog`].
//!
//! [`MemoryCatalog`] is always there. With the `file` feature on,
//! `FileCatalog` keeps products in a text file, one JSON object per line.

mod memory;
pub use memory::MemoryCatalog;

#[cfg(feature = "file")]
mod file;
#[cfg(feature = "file")]
pub use file::FileCatalog;
