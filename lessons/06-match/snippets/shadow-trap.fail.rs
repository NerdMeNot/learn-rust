#![deny(unreachable_patterns)]

fn main() {
    let limit = 10;
    for n in [3, 10, 42] {
        match n {
            limit => println!("{n}: at the limit"),
            _ => println!("{n}: not at the limit"),
        }
    }
}
