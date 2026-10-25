use std::collections::{BTreeMap, HashSet};

mod shop {
    pub const TAX_PERCENT: u32 = 10;

    pub mod prices {
        // a path starting at `crate::` begins at the top of the program
        pub fn with_tax(cents: u32) -> u32 {
            cents + cents * crate::shop::TAX_PERCENT / 100
        }

        // `super::` starts one module up
        pub fn doubled_with_tax(cents: u32) -> u32 {
            with_tax(super::double(cents))
        }
    }

    fn double(n: u32) -> u32 {
        n * 2
    }

    // visible in this program, but not meant for outside code
    pub(crate) fn describe() -> String {
        // `self::` starts in the current module
        format!("tax is {}%", self::TAX_PERCENT)
    }
}

use shop::prices::with_tax as taxed;

fn main() {
    let mut ages = BTreeMap::new();
    ages.insert("Ada", 36);
    let seen: HashSet<i32> = HashSet::new();

    println!("taxed: {}", taxed(1000));
    println!("doubled: {}", shop::prices::doubled_with_tax(1000));
    println!("describe: {}", shop::describe());
    println!("sizes: {} {}", ages.len(), seen.len());
    println!("method vs path: {} {}", "abc".len(), str::len("abc"));
}
