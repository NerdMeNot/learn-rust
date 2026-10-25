use std::rc::Rc;

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn main() {
    let first = Rc::new(Noisy("config"));
    println!("owners: {}", Rc::strong_count(&first));

    let second = Rc::clone(&first); // a second owner. Nothing is copied but a pointer.
    println!("owners: {}", Rc::strong_count(&first));
    println!("same value: {}", Rc::ptr_eq(&first, &second));
    println!("weak_count: {}", Rc::weak_count(&first));
    println!("size_of Rc<Noisy>: {} bytes", size_of::<Rc<Noisy>>());

    drop(first);
    println!("dropped first, owners: {}", Rc::strong_count(&second));

    drop(second); // the last owner goes, so the value is freed
    println!("end of main");
}
