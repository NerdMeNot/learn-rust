//! Count the words in a piece of text.
//!
//! ```
//! let stats = wordstat::count_words("the cat and the dog")?;
//! assert_eq!(stats.total(), 5);
//! assert_eq!(stats.get("the"), 2);
//! # Ok::<(), wordstat::Error>(())
//! ```

mod count;
mod error;
#[cfg(feature = "json")]
mod json;

pub use count::{Stats, count_words};
pub use error::Error;
