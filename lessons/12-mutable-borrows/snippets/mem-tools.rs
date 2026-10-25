use std::mem;

fn main() {
    let mut a = String::from("left");
    let mut b = String::from("right");
    mem::swap(&mut a, &mut b);
    println!("swap:    a = {a}, b = {b}");

    let mut title = String::from("draft");
    let old = mem::replace(&mut title, String::from("final"));
    println!("replace: old = {old}, title = {title}");

    let mut log = vec![1, 2, 3];
    let taken = mem::take(&mut log);
    println!("take:    taken = {taken:?}, log = {log:?}");
}
