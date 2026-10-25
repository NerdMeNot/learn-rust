struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropped {}", self.0);
    }
}

fn main() {
    let items = vec![Noisy("a"), Noisy("b"), Noisy("c")];

    for item in &items {
        println!("  saw {}", item.0);
        if item.0 == "b" {
            break;
        }
    }
    println!("loop is over, items still here: {}", items.len());
}
