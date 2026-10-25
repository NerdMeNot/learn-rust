fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn pick(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "mul" => Box::new(|a, b| a * b),
        _ => Box::new(|_, _| 0),
    }
}

fn main() {
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(1): {}, add10(1): {}", add5(1), add10(1));

    let mut a = make_counter();
    let mut b = make_counter();
    println!("a: {} {}, b: {}", a(), a(), b());

    for op in ["add", "mul", "other"] {
        println!("{op}(3, 4): {}", pick(op)(3, 4));
    }
}
