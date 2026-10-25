/// What to do with the part of a result that is smaller than one minor unit.
///
/// This enum is deliberately *not* `#[non_exhaustive]`, so the page can show
/// what adding a variant to such an enum does to its users.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rounding {
    /// Drop the fraction: 1.99 cents becomes 1 cent, -1.99 becomes -1.
    TowardZero,
    /// Round to the nearest minor unit; a half goes away from zero, so 0.5
    /// becomes 1 and -0.5 becomes -1 (not 0), and 1.49 becomes 1.
    HalfAwayFromZero,
    /// Round halves to the even neighbour.
    HalfEven,
}

impl Rounding {
    /// `n / d` for a positive `d`, rounded the way `self` says.
    pub(crate) fn divide(self, n: i128, d: i128) -> i128 {
        let quotient = n / d;
        let remainder = n % d;
        match self {
            Rounding::TowardZero => quotient,
            Rounding::HalfEven => {
                let twice = remainder.abs() * 2;
                if twice > d || (twice == d && quotient % 2 != 0) {
                    quotient + remainder.signum()
                } else {
                    quotient
                }
            }
            Rounding::HalfAwayFromZero => {
                if remainder.abs() * 2 >= d {
                    quotient + remainder.signum()
                } else {
                    quotient
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_modes_differ_on_a_half() {
        assert_eq!(Rounding::TowardZero.divide(250, 100), 2);
        assert_eq!(Rounding::HalfAwayFromZero.divide(250, 100), 3);
        assert_eq!(Rounding::HalfAwayFromZero.divide(-250, 100), -3);
        assert_eq!(Rounding::HalfAwayFromZero.divide(249, 100), 2);
    }

    #[test]
    fn half_away_from_zero_is_not_round_half_up() {
        // Rounding halves up would turn -1.5 into -1 and -2.5 into -2.
        assert_eq!(Rounding::HalfAwayFromZero.divide(150, 100), 2);
        assert_eq!(Rounding::HalfAwayFromZero.divide(-150, 100), -2);
        assert_eq!(Rounding::HalfAwayFromZero.divide(-250, 100), -3);
        assert_eq!(Rounding::HalfAwayFromZero.divide(-251, 100), -3);
        assert_eq!(Rounding::HalfAwayFromZero.divide(-249, 100), -2);
    }
}
