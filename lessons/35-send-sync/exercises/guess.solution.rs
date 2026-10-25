#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

struct Config {
    name: String,
    retries: Cell<u32>,
}

struct Handle {
    id: u32,
    cache: Arc<Mutex<Vec<String>>>,
}

/// For each type, in this order, say `(is it Send, is it Sync)`:
///   0 Config                      5 Arc<Mutex<Cell<i32>>>
///   1 Handle                      6 &mut Cell<i32>
///   2 Option<Rc<i32>>             7 Box<dyn Fn() + Sync>
///   3 (i32, *const u8)            8 Vec<MutexGuard<'_, i32>>
///   4 Mutex<Rc<i32>>              9 &Mutex<Cell<i32>>
fn guesses() -> [(bool, bool); 10] {
    [
        (true, false),  // Config: a String and a Cell; the Cell may move but not be shared
        (true, true),   // Handle: a u32 and an Arc<Mutex<Vec<String>>>, which is both
        (false, false), // Option<Rc<..>> is as good as its Rc
        (false, false), // a raw pointer is neither, and a tuple needs every part
        (false, false), // Mutex<T> is Sync only if T is Send; an Rc is not
        (true, true),   // Mutex<Cell<..>> is Sync (a Cell is Send); the Arc then needs both
        (true, false),  // &mut T: Send if T is Send, Sync if T is Sync; a Cell is only the first
        (false, true),  // the trait object says Sync and nothing about Send
        (false, true),  // a MutexGuard is Sync but not Send; a Vec is as good as its elements
        (true, true),   // &T is Send when T is Sync, and Mutex<Cell<..>> is Sync
    ]
}

fn main() {
    // These probes belong to the checks: they are defined inside `main`, so `guesses`
    // cannot call them. Predict first, then run.
    macro_rules! probe {
        ($name:ident, $bound:ident) => {
            macro_rules! $name {
                ($t:ty) => {{
                    struct Probe<T: ?Sized>(PhantomData<T>);
                    trait No {
                        fn answer(&self) -> bool {
                            false
                        }
                    }
                    impl<T: ?Sized> No for Probe<T> {}
                    impl<T: ?Sized + $bound> Probe<T> {
                        fn answer(&self) -> bool {
                            true
                        }
                    }
                    Probe::<$t>(PhantomData).answer()
                }};
            }
        };
    }
    probe!(is_send, Send);
    probe!(is_sync, Sync);

    macro_rules! actual {
        ($t:ty) => {
            (is_send!($t), is_sync!($t))
        };
    }
    let actual = [
        actual!(Config),
        actual!(Handle),
        actual!(Option<Rc<i32>>),
        actual!((i32, *const u8)),
        actual!(Mutex<Rc<i32>>),
        actual!(Arc<Mutex<Cell<i32>>>),
        actual!(&'static mut Cell<i32>),
        actual!(Box<dyn Fn() + Sync>),
        actual!(Vec<MutexGuard<'static, i32>>),
        actual!(&'static Mutex<Cell<i32>>),
    ];
    let names = [
        "Config",
        "Handle",
        "Option<Rc<i32>>",
        "(i32, *const u8)",
        "Mutex<Rc<i32>>",
        "Arc<Mutex<Cell<i32>>>",
        "&mut Cell<i32>",
        "Box<dyn Fn() + Sync>",
        "Vec<MutexGuard<'_, i32>>",
        "&Mutex<Cell<i32>>",
    ];

    let guessed = guesses();
    for i in 0..names.len() {
        // `assert!`, not `assert_eq!`: a failure must not print the right answer
        assert!(
            guessed[i] == actual[i],
            "wrong guess for {}: look at both Send and Sync",
            names[i]
        );
    }
    println!("all checks passed");
}
