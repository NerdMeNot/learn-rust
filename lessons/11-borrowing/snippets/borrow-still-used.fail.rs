fn main() {
    let name = String::from("Ada");

    let r = &name; // the borrow starts
    println!("r: {r}");

    let moved = name; // name moves while r is still going to be used
    println!("moved: {moved}");

    println!("r again: {r}"); // the borrow is still alive here
}
