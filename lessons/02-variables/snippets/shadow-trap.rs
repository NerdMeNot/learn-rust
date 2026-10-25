#![allow(unused)] // warnings off, to show what the program does

fn main() {
    let mut total = 0;
    for n in 1..=3 {
        let total = total + n; // meant: total += n
    }
    println!("total: {total}");
}
