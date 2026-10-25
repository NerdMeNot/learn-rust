use std::rc::Rc;

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let first = Rc::new(Noisy("first"));
    let second = Rc::new(Noisy("second"));
    let extra = Rc::clone(&first);
    drop(first);
    println!("dropped first");
    let boxed = Box::new(Noisy("boxed"));
    println!("end of main: {} {} {}", second.0, extra.0, boxed.0);
}
