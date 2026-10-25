#![forbid(unsafe_code)]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// Counts the steps the Collatz sequence takes to reach 1 (halve an even number,
/// triple an odd one and add 1), and remembers every answer it works out.
struct Collatz {
    known: RefCell<HashMap<u64, u32>>,
    computed: Cell<u32>, // how many numbers were worked out, rather than found in `known`
}

impl Collatz {
    fn new() -> Collatz {
        Collatz {
            known: RefCell::new(HashMap::new()),
            computed: Cell::new(0),
        }
    }

    /// Steps from `n` to 1. Takes `&self`: callers share one Collatz.
    fn steps(&self, n: u64) -> u32 {
        let cached = self.known.borrow().get(&n).copied();
        if let Some(steps) = cached {
            return steps;
        }
        self.computed.set(self.computed.get() + 1);
        let steps = if n == 1 {
            0
        } else if n % 2 == 0 {
            1 + self.steps(n / 2)
        } else {
            1 + self.steps(3 * n + 1)
        };
        self.known.borrow_mut().insert(n, steps);
        steps
    }
}

fn main() {
    let c = Collatz::new();
    assert_eq!(c.steps(1), 0);
    assert_eq!(c.computed.get(), 1);

    // 27 takes 111 steps and passes through 112 different numbers, 1 among them
    let c = Collatz::new();
    assert_eq!(c.steps(27), 111);
    assert_eq!(c.computed.get(), 112);

    // asking again finds everything in the memo
    assert_eq!(c.steps(27), 111);
    assert_eq!(
        c.computed.get(),
        112,
        "the second call must use the stored answers"
    );

    // 54 is 27 doubled: one new number
    assert_eq!(c.steps(54), 112);
    assert_eq!(c.computed.get(), 113);

    // a long chain works too: 525 numbers, so 525 nested calls
    assert_eq!(c.steps(837799), 524);

    println!("all checks passed");
}
