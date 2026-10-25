struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropped {}", self.0);
    }
}

fn main() {
    let items = vec![Noisy("a"), Noisy("b"), Noisy("c"), Noisy("d")];

    println!("loop starts");
    for item in items {
        println!("  got {}", item.0);
        if item.0 == "b" {
            println!("  break");
            break;
        }
    }
    println!("loop is over");
}
