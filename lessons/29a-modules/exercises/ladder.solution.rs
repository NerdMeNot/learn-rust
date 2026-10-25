mod store {
    // No `pub`: visible in `store` and in every module inside it.
    fn currency(region: &str) -> &'static str {
        match region {
            "fr" => "EUR",
            "us" => "USD",
            _ => "GBP",
        }
    }

    pub mod util {
        // Cents rounded to whole units. Visible in `store`, not beyond.
        pub(super) fn round(cents: u32) -> u32 {
            (cents + 50) / 100
        }
    }

    pub mod report {
        // "tea: 3 EUR" for 250 cents in "fr". Use `util::round` and `currency`.
        // Calling `round` while it is private to `util` is E0603: read it first.
        pub fn line(name: &str, cents: u32, region: &str) -> String {
            format!(
                "{name}: {} {}",
                super::util::round(cents),
                super::currency(region)
            )
        }
    }
}

// `rival` also has a `round`. With both globs below, `round(250)` in `main` is
// ambiguous (E0659) when `store::util::round` is visible here. Pick the narrowest
// visibility that still lets `report` call it, and the ambiguity goes away.
mod rival {
    pub fn round(cents: u32) -> u32 {
        cents / 100
    }
}

#[allow(unused_imports)]
use rival::*;
#[allow(unused_imports)]
use store::util::*;

fn main() {
    assert_eq!(round(250), 2);
    assert_eq!(store::report::line("tea", 250, "fr"), "tea: 3 EUR");
    assert_eq!(store::report::line("pen", 99, "us"), "pen: 1 USD");
    assert_eq!(store::report::line("ale", 1000, "uk"), "ale: 10 GBP");
    println!("ok");
}
