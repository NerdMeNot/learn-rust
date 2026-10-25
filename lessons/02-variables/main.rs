fn main() {
    let name = "gopher";      // cannot change
    let mut count = 1;        // mut: can change
    count += 1;

    let count = count * 10;   // shadowing: a NEW variable with the same name
    println!("{name} {count}");
}
