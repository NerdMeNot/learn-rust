fn main() {
    let name = String::from("Ada");

    let r = &name; // the borrow starts
    println!("r: {r}"); // the last use of r: the borrow ends here

    let moved = name; // moving name is fine: nothing borrows it any more
    println!("moved: {moved}");
}
