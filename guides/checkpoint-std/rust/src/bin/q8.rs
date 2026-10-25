struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("guard dropped");
    }
}

fn main() {
    let _guard = Guard;
    println!("working");
    std::process::exit(3);
}
