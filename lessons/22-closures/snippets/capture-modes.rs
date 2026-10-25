fn main() {
    // reads `factor`: the closure holds a shared reference to it
    let factor = 3;
    let times = |x: i32| x * factor;
    println!("times(5): {}", times(5));
    println!("factor, read beside the closure: {factor}");

    // changes `count`: the closure holds a mutable reference to it
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    inc();
    println!("count after two calls: {count}");

    // gives `name` away: the closure takes it
    let name = String::from("ferris");
    let consume = || drop(name);
    consume();
    println!("the closure has run once; it cannot run again");
}
