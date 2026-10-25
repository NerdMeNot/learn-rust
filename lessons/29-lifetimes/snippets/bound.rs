// 'b: 'a reads "'b outlives 'a". It lets a &'b str be returned as a &'a str.
fn pick<'a, 'b: 'a>(x: &'a str, y: &'b str, first: bool) -> &'a str {
    if first { x } else { y }
}

// The compiler infers T: 'a here; you do not write it.
struct Holder<'a, T> {
    value: &'a T,
}

fn main() {
    let x = String::from("x");
    let y = String::from("y");
    println!("pick: {}", pick(&x, &y, false));
    let holder = Holder { value: &x };
    println!("holder: {}", holder.value);
}
