mod temperature {
    /// A temperature in degrees Celsius: a finite number, not below absolute zero (-273.15).
    /// The field is private, so `try_from` is the only way to make one.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Celsius(f64);

    #[derive(Debug, PartialEq)]
    pub enum TempError {
        NotFinite,
        BelowAbsoluteZero,
    }

    impl TryFrom<f64> for Celsius {
        type Error = TempError;

        fn try_from(degrees: f64) -> Result<Celsius, TempError> {
            todo!("reject NaN and both infinities, then anything below -273.15")
        }
    }

    impl Celsius {
        pub fn degrees(self) -> f64 {
            self.0
        }
    }
}

use temperature::{Celsius, TempError};

fn main() {
    assert_eq!(Celsius::try_from(21.5).unwrap().degrees(), 21.5);
    assert_eq!(Celsius::try_from(-273.15).unwrap().degrees(), -273.15); // the limit itself is allowed
    assert_eq!(
        Celsius::try_from(-273.16),
        Err(TempError::BelowAbsoluteZero)
    );
    assert_eq!(Celsius::try_from(-1e9), Err(TempError::BelowAbsoluteZero));
    assert_eq!(Celsius::try_from(f64::NAN), Err(TempError::NotFinite));
    assert_eq!(Celsius::try_from(f64::INFINITY), Err(TempError::NotFinite));
    assert_eq!(
        Celsius::try_from(f64::NEG_INFINITY),
        Err(TempError::NotFinite)
    );
    // `try_into` comes for free with `TryFrom`.
    let boiling: Result<Celsius, TempError> = 100.0_f64.try_into();
    assert_eq!(boiling.map(Celsius::degrees), Ok(100.0));
    println!("all checks passed");
}
