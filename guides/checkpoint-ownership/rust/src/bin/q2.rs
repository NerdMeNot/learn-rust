struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn take(n: Noisy) {
    println!("took {}", n.0);
}

fn main() {
    let _a = Noisy("a");
    let a2 = Noisy("a2");
    let _ = Noisy("b");
    let _c = Noisy("c");
    let _a = Noisy("shadow");
    take(a2);
    println!("end");
}
