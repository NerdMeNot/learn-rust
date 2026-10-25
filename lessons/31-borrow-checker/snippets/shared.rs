use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Settings {
    volume: u8,
}

fn main() {
    // clone: two independent copies. Changing one leaves the other alone.
    let original = Settings { volume: 3 };
    let mut copy = original.clone();
    copy.volume = 9;
    println!("clone: original {}, copy {}", original.volume, copy.volume);

    // Rc<RefCell<_>>: one value, two handles. A change through either is seen by both.
    let shared = Rc::new(RefCell::new(Settings { volume: 3 }));
    let other = Rc::clone(&shared);
    other.borrow_mut().volume = 9;
    println!(
        "shared: {} and {}",
        shared.borrow().volume,
        other.borrow().volume
    );
    println!("owners: {}", Rc::strong_count(&shared));
}
