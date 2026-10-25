fn describe(items: &[i32]) {
    match items {
        [] => println!("{items:?}: empty"),
        [only] => println!("{items:?}: just {only}"),
        [a, b] => println!("{items:?}: a pair, {a} and {b}"),
        [first, .., last] => println!("{items:?}: from {first} to {last}"),
    }
}

fn main() {
    describe(&[]);
    describe(&[7]);
    describe(&[3, 4]);
    describe(&[1, 2, 3, 4]);
}
