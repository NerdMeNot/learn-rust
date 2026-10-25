fn take<T>(_value: T) {}

fn main() {
    let mut n = 0;
    let r = &mut n;
    take(r); // a generic parameter has no known `&mut` target: this moves `r`
    *r += 1;
    println!("n: {n}");
}
