fn total(n: i32) -> i32 {
    if n > 0 { 10 } else { 20 } + 1
}

fn main() {
    println!("{}", total(5));
}
