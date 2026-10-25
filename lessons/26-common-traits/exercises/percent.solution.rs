// A percentage is a whole number from 0 to 100.
#[derive(Debug, PartialEq)]
struct Percent(u8);

#[derive(Debug, PartialEq)]
struct OutOfRange;

impl TryFrom<i32> for Percent {
    type Error = OutOfRange;

    fn try_from(n: i32) -> Result<Percent, OutOfRange> {
        match u8::try_from(n) {
            Ok(small) if small <= 100 => Ok(Percent(small)),
            _ => Err(OutOfRange),
        }
    }
}

// 50 percent is 0.5.
impl From<Percent> for f64 {
    fn from(p: Percent) -> f64 {
        f64::from(p.0) / 100.0
    }
}

fn main() {
    assert_eq!(Percent::try_from(0), Ok(Percent(0)));
    assert_eq!(Percent::try_from(100), Ok(Percent(100)));
    assert_eq!(Percent::try_from(101), Err(OutOfRange));
    assert_eq!(Percent::try_from(-1), Err(OutOfRange));
    // These wrap into 0..=100 if you cast with `as u8` first: 256 -> 0, 300 -> 44, -255 -> 1.
    assert_eq!(Percent::try_from(256), Err(OutOfRange));
    assert_eq!(Percent::try_from(300), Err(OutOfRange));
    assert_eq!(Percent::try_from(-255), Err(OutOfRange));

    let p: Result<Percent, _> = 25_i32.try_into();
    assert_eq!(p, Ok(Percent(25)));
    assert_eq!(f64::from(Percent(50)), 0.5);
    let ratio: f64 = Percent(100).into();
    assert_eq!(ratio, 1.0);
    println!("all checks passed");
}
