// A pattern can sit in a parameter list.
fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    println!("manhattan: {}", manhattan((1, 2), (4, -2)));

    // Nested tuples unpack in one step.
    let ((a, b), label) = ((1, 2), "pair");
    println!("a: {a}, b: {b}, label: {label}");

    // `..` skips everything else.
    let four = (10, 20, 30, 40);
    let (first, ..) = four;
    let (.., last) = four;
    println!("first: {first}, last: {last}");

    // A `for` loop takes a pattern too.
    for (name, score) in [("ana", 3), ("bo", 5)] {
        println!("{name}: {score}");
    }
}
