struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let a = Noisy("list");
    {
        let b = a;
        println!("b owns {}", b.0);
    }
    println!("after the block");
}
