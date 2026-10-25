struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropped {}", self.0);
    }
}

// T: 'static means "T holds no borrow that could end early". It is not "T lives forever".
fn consume<T: 'static>(_value: T) {
    println!("consume returns");
}

fn main() {
    consume(String::from("an owned String is fine"));
    consume(Noisy("a"));
    println!("after consume");
}
