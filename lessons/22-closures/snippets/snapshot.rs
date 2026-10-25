fn main() {
    let mut x = 1;
    let show = move || println!("x in the closure: {x}");
    x = 2;
    show();
    println!("x outside: {x}");
}
