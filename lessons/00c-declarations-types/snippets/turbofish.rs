fn main() {
    let a = "42".parse::<i32>().unwrap(); // turbofish: ::<type>
    let b: i32 = "42".parse().unwrap(); // or annotate the variable
    let words = "a b c".split(' ').collect::<Vec<_>>(); // gather the pieces into a Vec
    let empty = Vec::<u8>::new();

    println!("a: {a}, b: {b}");
    println!("words: {words:?}, empty: {empty:?}");
}
