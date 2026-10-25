fn main() {
    // take_while has to pull the first failing item to see that it fails.
    // That item is then gone.
    let mut it = [1, 2, 10, 3, 4].into_iter();
    let small: Vec<i32> = it.by_ref().take_while(|&n| n < 5).collect();
    let rest: Vec<i32> = it.collect();
    println!("take_while: {small:?}, rest: {rest:?}"); // the 10 is lost

    // peekable + next_if looks first and leaves the failing item in place.
    let mut it = [1, 2, 10, 3, 4].into_iter().peekable();
    let mut small = Vec::new();
    while let Some(n) = it.next_if(|&n| n < 5) {
        small.push(n);
    }
    let rest: Vec<i32> = it.collect();
    println!("next_if: {small:?}, rest: {rest:?}");
}
