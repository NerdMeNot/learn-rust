use std::mem;
use std::rc::Rc;
use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(Rc::new(5));
    let (copy, taken) = {
        let mut guard = lock.lock().unwrap();
        let copy = Rc::clone(&guard); // a second owner of the same allocation
        let taken = mem::replace(&mut *guard, Rc::new(6)); // or the Rc itself, swapped out
        (copy, taken)
    }; // the guard is gone and the lock is free, but both Rcs are still alive
    println!("owners of the first Rc: {}", Rc::strong_count(&copy));
    println!("same allocation: {}", Rc::ptr_eq(&copy, &taken));
}
