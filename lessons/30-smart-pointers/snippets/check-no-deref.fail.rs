use std::cell::RefCell;
use std::rc::Weak;

fn main() {
    let cell = RefCell::new(String::from("text"));
    let weak: Weak<String> = Weak::new();
    println!("{}", cell.len()); // RefCell does not deref: only its guards do
    println!("{}", weak.len()); // Weak does not deref: upgrade() first
}
