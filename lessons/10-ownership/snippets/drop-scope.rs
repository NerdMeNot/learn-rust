// A type that says when it is freed. `Drop` is Rust's destructor: `drop` runs
// automatically when the owner goes away. You never call it yourself.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn main() {
    println!("start of main");
    let _outer = Noisy("outer");

    {
        println!("start of block");
        let _inner = Noisy("inner");
        println!("end of block");
    } // _inner's scope ends here, so it is dropped here

    println!("end of main");
} // _outer is dropped here
