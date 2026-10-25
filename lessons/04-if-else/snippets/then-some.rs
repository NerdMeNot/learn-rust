fn costly() -> i32 {
    println!("costly() ran");
    1
}

fn main() {
    let n = 12;
    // `then_some` turns a bool into an Option: Some(value) when true, None when false
    println!("{:?}", (n > 10).then_some("big"));
    println!("{:?}", (n > 20).then_some("huge"));

    // the argument is evaluated even when the bool is false
    let early = (n > 20).then_some(costly());
    println!("{:?}", early);

    // the same with an if expression
    let big = if n > 10 { Some("big") } else { None };
    println!("{:?}", big);
}
