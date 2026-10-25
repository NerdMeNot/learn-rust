fn main() {
    let n = 5;
    let r: &i32 = &n;

    // Arithmetic works through one &.
    println!("r + 1: {}", r + 1);

    // * follows the reference to the value. For a number that is a copy.
    let copy: i32 = *r;
    println!("*r: {copy}");

    // Comparing needs the same type on both sides.
    println!("r == &5: {}", r == &5);
    println!("*r == 5: {}", *r == 5);

    // == on two references compares the values they point at, not the addresses.
    let a = String::from("Ada");
    let b = String::from("Ada");
    println!("&a == &b: {}", &a == &b);
    println!("same address: {}", std::ptr::eq(&a, &b));

    // Printing shows the value, not the address.
    println!("r printed: {r}");

    // Method calls follow the reference for you.
    println!("r.pow(2): {}", r.pow(2));
    let rn: &String = &a;
    println!("rn.len(): {}", rn.len());
}
