#![allow(dead_code)]
// Backs every cell of the "std types" table on the page, positive and negative.
// `is_send!(T)` / `is_sync!(T)` ask whether the bound holds, as a bool: the inherent
// method is only callable when `T: Send`, so otherwise method lookup falls through
// to the trait's default `false`.
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

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

macro_rules! cell {
    ($t:ty, $send:expr, $sync:expr) => {
        assert_eq!(is_send!($t), $send, "Send for {}", stringify!($t));
        assert_eq!(is_sync!($t), $sync, "Sync for {}", stringify!($t));
    };
}

struct Plain {
    id: u32,
}

struct Pinned {
    id: u32,
    // `PhantomData<T>` counts as a `T` for Send and Sync; `*const ()` is neither
    _not_thread_safe: PhantomData<*const ()>,
}

fn main() {
    let (p, q) = (
        Plain { id: 1 },
        Pinned {
            id: 2,
            _not_thread_safe: PhantomData,
        },
    );
    assert_eq!(p.id + q.id, 3);
    cell!(Plain, true, true);
    cell!(Pinned, false, false);
    cell!(i32, true, true);
    cell!(String, true, true);
    cell!(Vec<String>, true, true);
    cell!(Rc<i32>, false, false);
    cell!(Arc<i32>, true, true);
    cell!(Cell<i32>, true, false);
    cell!(RefCell<i32>, true, false);
    cell!(Mutex<i32>, true, true);
    cell!(Mutex<Cell<i32>>, true, true);
    cell!(RwLock<Cell<i32>>, true, false);
    cell!(MutexGuard<'static, i32>, false, true);
    cell!(*const i32, false, false);
    cell!(*mut i32, false, false);
    cell!(&'static i32, true, true);
    cell!(&'static Cell<i32>, false, false);
    cell!(&'static mut i32, true, true);
    cell!(&'static mut Cell<i32>, true, false);
    cell!(&'static mut Rc<i32>, false, false);
    cell!(Arc<RefCell<i32>>, false, false);
    cell!(Arc<Mutex<i32>>, true, true);
    cell!(Arc<Vec<String>>, true, true);
    cell!(Mutex<Rc<i32>>, false, false);
    cell!(Arc<MutexGuard<'static, i32>>, false, false);
    cell!(Arc<Rc<i32>>, false, false);
    cell!(Rc<Mutex<i32>>, false, false);
    cell!(Box<dyn Fn()>, false, false);
    cell!(Box<dyn Fn() + Send>, true, false);
    cell!(Box<dyn Fn() + Send + Sync>, true, true);
    cell!(Sender<i32>, true, true);
    cell!(Receiver<i32>, true, false);
    cell!(Vec<Rc<i32>>, false, false);
    cell!(Option<Cell<i32>>, true, false);
    cell!((i32, Rc<i32>), false, false);
    cell!(PhantomData<*const ()>, false, false);

    // `&T` is Send exactly when `T` is Sync
    macro_rules! ref_rule {
        ($t:ty) => {
            assert_eq!(
                is_send!(&'static $t),
                is_sync!($t),
                "&T: Send iff T: Sync for {}",
                stringify!($t)
            );
        };
    }
    ref_rule!(i32);
    ref_rule!(Cell<i32>);
    ref_rule!(RefCell<i32>);
    ref_rule!(Mutex<i32>);
    ref_rule!(Rc<i32>);
    ref_rule!(Arc<i32>);
    ref_rule!(MutexGuard<'static, i32>);
    ref_rule!(*const i32);
    ref_rule!(Receiver<i32>);

    println!("every cell of the table holds");
}
