fn main() {
    let factor = 3;
    let times = |x: i32| x * factor;     // reads factor
    println!("{}", times(5));

    let mut count = 0;
    let mut inc = || count += 1;         // changes count, so inc is `mut`
    inc();
    inc();
    println!("count={count}");
}
