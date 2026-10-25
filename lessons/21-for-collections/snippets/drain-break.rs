struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropped {}", self.0);
    }
}

fn main() {
    let mut items = vec![Noisy("a"), Noisy("b"), Noisy("c")];

    for item in items.drain(..) {
        println!("  got {}", item.0);
        if item.0 == "b" {
            break;
        }
    }
    println!("loop is over, items left: {}", items.len());
}
