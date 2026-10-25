use rayon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let total = Rc::new(RefCell::new(0u64));
    (1..=1_000_000u64).into_par_iter().for_each(|n| {
        *total.borrow_mut() += n;
    });
    println!("{}", total.borrow());
}
