use rayon::prelude::*;
use std::panic::catch_unwind;

fn main() {
    // Keep stderr quiet so the output is the same on every run.
    std::panic::set_hook(Box::new(|_| {}));

    let result = catch_unwind(|| {
        (1..=1_000).into_par_iter().for_each(|n| {
            if n == 500 {
                panic!("bad item {n}");
            }
        });
    });
    match result {
        Ok(()) => println!("no panic reached the caller"),
        Err(payload) => {
            let message = payload.downcast_ref::<String>().unwrap();
            println!("the caller got the panic: {message}");
        }
    }

    // The pool is still usable afterwards.
    let total: u32 = (1..=10).into_par_iter().sum();
    println!("a later par_iter: {total}");
}
