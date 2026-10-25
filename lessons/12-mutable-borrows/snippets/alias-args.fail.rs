fn add_twice(total: &mut i32, amount: &i32) {
    *total += *amount;
    *total += *amount;
}

fn main() {
    let mut x = 1;
    add_twice(&mut x, &x);
    println!("x: {x}");
}
