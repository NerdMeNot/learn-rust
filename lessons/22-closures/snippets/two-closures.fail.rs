fn pick(add: bool, n: i32) -> impl Fn(i32) -> i32 {
    if add { move |x| x + n } else { move |x| x * n }
}

fn main() {
    println!("{}", pick(true, 5)(1));
}
