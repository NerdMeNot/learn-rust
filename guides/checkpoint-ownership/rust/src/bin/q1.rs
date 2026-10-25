fn consume(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");
    for _ in 0..2 {
        println!("{}", consume(s));
    }
}
