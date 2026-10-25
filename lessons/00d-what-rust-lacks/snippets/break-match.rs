fn main() {
    let mut n = 0;
    for i in 0..5 {
        match i {
            2 => break,
            _ => n += 1,
        }
    }
    println!("n: {n}");
}
