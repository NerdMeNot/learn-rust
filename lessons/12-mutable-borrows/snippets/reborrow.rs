fn add_one(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut n = 0;
    let r = &mut n;
    add_one(r); // `r` is lent for the call, not used up
    add_one(&mut *r); // the same thing, written out
    *r += 10;
    println!("n: {n}");
}
