fn main() {
    let letters = ['a', 'b', 'c'];
    for (i, c) in letters.iter().rev().enumerate() {
        println!("{i} {c}");
    }
    for (i, c) in letters.iter().enumerate().rev() {
        println!("{i} {c}");
    }
}
