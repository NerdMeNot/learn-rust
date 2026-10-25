struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropping {}", self.0);
    }
}

fn main() {
    let outer = Noisy("outer");
    let r = &outer;
    println!("r points at outer: {}", std::ptr::eq(r, &outer));
    println!("using r: {}", r.0);
    println!("the borrow of outer is over; outer lives on");

    {
        let inner = Noisy("inner");
        let r2 = &inner;
        println!("using r2: {}", r2.0);
        println!("end of the inner block");
    }
    println!("after the inner block");
}
