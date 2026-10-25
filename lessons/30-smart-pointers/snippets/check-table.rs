use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Box<T>: one owner, one pointer wide, moving it leaves the heap value where it is
    let boxed = Box::new([1u8; 512]);
    let at = boxed.as_ptr();
    let moved = boxed;
    assert_eq!(moved.as_ptr(), at);
    let back = *moved; // a Box lets the value be moved back out; an Rc does not
    assert_eq!(back[0], 1);
    assert_eq!(size_of::<Box<[u8; 512]>>(), size_of::<usize>());

    // Rc<T>: several owners on one thread, read-only
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    assert_eq!(Rc::strong_count(&a), 2);
    assert!(Rc::ptr_eq(&a, &b));
    assert_eq!(a.as_ptr(), b.as_ptr());

    // Arc<T>: the same, with a count that threads may update
    let n = Arc::new(5);
    let n2 = Arc::clone(&n);
    assert_eq!(thread::spawn(move || *n2 + 1).join().unwrap(), 6);
    assert_eq!(Arc::strong_count(&n), 1);

    // Cell<T>: a Copy value changed through &
    let c = Cell::new(1);
    let r = &c;
    r.set(r.get() + 1);
    assert_eq!(c.get(), 2);

    // RefCell<T>: any value changed through &, checked when it runs
    let rc = RefCell::new(vec![1]);
    let reading = rc.borrow();
    assert!(rc.try_borrow_mut().is_err());
    drop(reading);
    rc.borrow_mut().push(2);
    assert_eq!(*rc.borrow(), vec![1, 2]);

    // Mutex<T>: changed from several threads, one at a time
    let total = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let total = Arc::clone(&total);
            thread::spawn(move || *total.lock().unwrap() += 1)
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(*total.lock().unwrap(), 4);

    // Rc<RefCell<T>>: shared and changeable, one thread
    let x = Rc::new(RefCell::new(0));
    let y = Rc::clone(&x);
    *y.borrow_mut() += 5;
    assert_eq!(*x.borrow(), 5);

    // Weak<T>: a link that does not keep the value alive
    let strong = Rc::new(1);
    let weak: Weak<i32> = Rc::downgrade(&strong);
    assert_eq!(weak.upgrade().as_deref(), Some(&1));
    drop(strong);
    assert!(weak.upgrade().is_none());

    println!("all table facts hold");
}
