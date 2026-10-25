fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let a = { double(4) };
    let b = {
        double(4);
    };
    println!("a: {a:?}");
    println!("b: {b:?}");
}
