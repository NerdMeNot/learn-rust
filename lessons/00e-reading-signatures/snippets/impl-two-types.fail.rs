fn pick(n: i32, times: bool) -> impl Fn(i32) -> i32 {
    if times {
        move |x| x * n
    } else {
        move |x| x + n
    }
}

fn main() {
    println!("pick: {}", pick(3, true)(5));
}
