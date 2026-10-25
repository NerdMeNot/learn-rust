mod percent {
    /// A number from 0 to 100. The field has no `pub`, so code outside this
    /// module can only get a `Percent` through the functions below.
    pub struct Percent(u8);

    impl Percent {
        pub fn new(n: u8) -> Option<Percent> {
            if n <= 100 { Some(Percent(n)) } else { None }
        }

        pub fn get(&self) -> u8 {
            self.0
        }

        pub fn of(&self, amount: u32) -> u32 {
            amount * self.0 as u32 / 100
        }

        /// The sum of two percentages. It must never go past 100.
        pub fn add(&self, other: &Percent) -> Percent {
            Percent((self.0 + other.0).min(100))
        }
    }

    /// Reading text such as "25%". A child module sees the private field of
    /// `Percent`, so it can write `Percent(n)` itself. Should it?
    pub mod parse {
        use super::Percent;

        pub fn from_text(s: &str) -> Option<Percent> {
            let n: u8 = s.strip_suffix('%')?.parse().ok()?;
            Percent::new(n)
        }
    }
}

// Outside `percent`: the field is not visible here, only the functions.
mod report {
    use crate::percent::Percent;

    pub fn show(p: &Percent) -> String {
        format!("{}%", p.get())
    }
}

use percent::{Percent, parse};

fn main() {
    assert!(Percent::new(101).is_none());
    assert!(Percent::new(255).is_none());
    assert_eq!(Percent::new(0).unwrap().get(), 0);
    assert_eq!(Percent::new(100).unwrap().get(), 100);
    assert_eq!(Percent::new(25).unwrap().of(200), 50);
    // Round down after multiplying: 33% of 50 is 16.5.
    assert_eq!(Percent::new(33).unwrap().of(50), 16);
    let sum = Percent::new(30).unwrap().add(&Percent::new(20).unwrap());
    assert_eq!(sum.get(), 50);
    let capped = Percent::new(60).unwrap().add(&Percent::new(70).unwrap());
    assert_eq!(capped.get(), 100);
    // Text goes through the same gate as numbers.
    assert_eq!(report::show(&parse::from_text("25%").unwrap()), "25%");
    assert!(parse::from_text("100%").is_some());
    assert!(parse::from_text("150%").is_none());
    assert!(parse::from_text("25").is_none());
    assert!(parse::from_text("x%").is_none());
    println!("ok");
}
