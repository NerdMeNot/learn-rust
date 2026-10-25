macro_rules! show {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e)
    };
}

// rustfmt would respace `a+b`, which is the point of the second line
#[rustfmt::skip]
fn main() {
    let (a, b) = (1, 2);
    show!(a + b);
    show!(a+b);
    show!(a * (b + 3));
    show!(vec![a, b].len());
}
