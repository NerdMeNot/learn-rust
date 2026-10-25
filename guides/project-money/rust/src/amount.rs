/// A count of the smallest unit of a currency: cents for dollars, yen for yen.
///
/// The field is private. Code outside this crate has to say what the number
/// means by calling [`Amount::from_minor`], so a bare `i64` cannot be passed
/// where an `Amount` is expected.
///
/// ```
/// use tinymoney::Amount;
///
/// let fee = Amount::from_minor(250);
/// assert_eq!(fee.minor(), 250);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount(i64);

impl Amount {
    /// No money at all.
    pub const ZERO: Amount = Amount(0);

    /// An amount of `minor` units: `250` is 2.50 dollars.
    pub fn from_minor(minor: i64) -> Amount {
        Amount(minor)
    }

    /// The number of minor units.
    pub fn minor(self) -> i64 {
        self.0
    }

    /// The sum, or `None` if it does not fit in an `i64`.
    pub(crate) fn checked_add(self, other: Amount) -> Option<Amount> {
        self.0.checked_add(other.0).map(Amount)
    }
}
