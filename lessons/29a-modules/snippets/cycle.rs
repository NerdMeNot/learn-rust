mod even {
    pub fn is_even(n: u32) -> bool {
        n == 0 || crate::odd::is_odd(n - 1)
    }
}

mod odd {
    pub fn is_odd(n: u32) -> bool {
        n != 0 && crate::even::is_even(n - 1)
    }
}

fn main() {
    println!("is_even(10): {}", even::is_even(10));
    println!("is_odd(7): {}", odd::is_odd(7));
}
