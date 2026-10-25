struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // let takes a value apart, by field name
    let Point { x, y } = Point { x: 3, y: -2 };
    println!("x = {x}, y = {y}");

    // so does a for loop, one item at a time
    for (i, c) in "abc".chars().enumerate() {
        println!("{i}: {c}");
    }

    // if let takes a value apart only when it fits
    if let Some((key, value)) = "mode=fast".split_once('=') {
        println!("{key} -> {value}");
    }

    // [first, .., last] names the ends of an array
    let [first, .., last] = [10, 20, 30, 40];
    println!("first = {first}, last = {last}");
}
