// The warnings are silenced here so the program builds cleanly; the next example shows them.
#![allow(unreachable_patterns, unused_variables)]

fn main() {
    let limit = 10;
    for n in [3, 10, 42] {
        match n {
            limit => println!("{n}: at the limit"),
            _ => println!("{n}: not at the limit"),
        }
    }
}
