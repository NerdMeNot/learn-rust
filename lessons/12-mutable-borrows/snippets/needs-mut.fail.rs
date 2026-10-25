fn add_one(x: &mut i32) {
    *x += 1;
}

fn main() {
    let n = 5;
    add_one(&mut n);
    println!("n: {n}");
}
