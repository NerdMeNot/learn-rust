fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn next(counter: &mut i32) -> i32 {
    *counter += 1;
    *counter
}

fn main() {
    let mut i = 5;
    let j = 3;
    let m = largest(next(&mut i), j); // next runs once
    println!("m = {m}, i = {i}");
    println!("{}", largest("pear", "apple"));
}
