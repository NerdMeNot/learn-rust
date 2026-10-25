fn sign(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    if n == 0 {
        return 0;
    }
    1
}

fn next_multiple(of: u32, above: u32) -> u32 {
    let mut n = above + 1;
    loop {
        if n % of == 0 {
            return n;
        }
        n += 1;
    }
}

fn main() {
    println!("sign(-7): {}", sign(-7));
    println!("sign(0): {}", sign(0));
    println!("sign(12): {}", sign(12));
    println!("next_multiple(7, 20): {}", next_multiple(7, 20));
}
