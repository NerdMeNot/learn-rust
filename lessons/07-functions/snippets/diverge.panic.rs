fn fail(msg: &str) -> ! {
    panic!("{msg}")
}

fn half(n: i32) -> i32 {
    if n % 2 == 0 {
        n / 2
    } else {
        fail("odd number")
    }
}

fn main() {
    println!("half(10): {}", half(10));
    println!("half(7): {}", half(7));
}
