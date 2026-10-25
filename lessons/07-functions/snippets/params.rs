fn bump(mut n: i32) -> i32 {
    n += 1;
    n
}

fn cap(qty: u32) -> u32 {
    let qty = qty.min(10); // a new variable that hides the parameter
    qty * 3
}

fn main() {
    let a = 5;
    let b = bump(a);
    println!("a: {a}, b: {b}");
    println!("cap(4): {}", cap(4));
    println!("cap(50): {}", cap(50));
}
