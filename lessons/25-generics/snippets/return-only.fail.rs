fn zero<T: Default>() -> T {
    T::default()
}

fn main() {
    let n = zero();
    println!("{n}");
}
