struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let value = Noisy("value");
    let r = &value; // a reference reaches the value and owns nothing
    println!("through r: {}", r.0);
    {
        let b = Box::new(Noisy("boxed")); // owns a value on the heap
        println!("through b: {}", b.0); // reaches it like a reference
        println!("size_of &Noisy: {}", size_of::<&Noisy>());
        println!("size_of Box<Noisy>: {}", size_of::<Box<Noisy>>());
    } // b goes: its Noisy is dropped and the heap memory freed
    println!("after the block");
} // r goes and nothing happens; value is dropped here
