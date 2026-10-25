struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

struct Pair {
    first: Noisy,
    second: Noisy,
}

fn main() {
    let _x = Noisy("x");
    let _ = Noisy("ignored");
    let pair = Pair { first: Noisy("first"), second: Noisy("second") };
    let _y = Noisy("y");
    println!("pair: {} and {}", pair.first.0, pair.second.0);
    println!("end of main");
}
