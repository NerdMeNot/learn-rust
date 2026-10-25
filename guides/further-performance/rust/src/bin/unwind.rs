use std::panic;

struct Noisy;

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy dropped");
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {})); // keep the output about the setting, not the message
    let result = panic::catch_unwind(|| {
        let _guard = Noisy;
        panic!("boom");
    });
    println!("caught: {}", result.is_err());
}
