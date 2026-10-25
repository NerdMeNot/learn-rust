fn main() {
    let mut x = 1;
    let show = || println!("x: {x}");
    x = 2;
    show();
}
