fn square(n: i32) -> i32 {
    let result = n * n;
    println!("  in square: n = {n}, result = {result}");
    result
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    println!("in sum_of_squares: a = {a}, b = {b}");
    let first = square(a);
    let second = square(b);
    first + second
}

fn main() {
    let x = 3;
    let y = 4;
    println!("in main: x = {x}, y = {y}");
    let total = sum_of_squares(x, y);
    println!("in main: total = {total}");
}
