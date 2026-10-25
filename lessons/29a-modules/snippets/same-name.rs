mod a {
    pub fn helper() -> u32 {
        1
    }
}

mod b {
    pub fn helper() -> u32 {
        2
    }
}

fn main() {
    // Two items called `helper`: each lives at its own path.
    println!("a::helper(): {}", a::helper());
    println!("b::helper(): {}", b::helper());
}
