// Neither closure captures anything, so each one coerces to a plain `fn` pointer
// and the two arms have the same type.
fn pick(add: bool) -> fn(i32) -> i32 {
    if add { |x| x + 1 } else { |x| x * 2 }
}

fn main() {
    println!("pick(true)(5): {}", pick(true)(5));
    println!("pick(false)(5): {}", pick(false)(5));
    let both = [|x: i32| x + 1, |x: i32| x * 2];
    println!("both[1](5): {}", both[1](5));
}
