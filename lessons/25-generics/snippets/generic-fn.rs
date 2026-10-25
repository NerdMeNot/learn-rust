// T is a placeholder: the compiler fills it in at each call.
fn first_and_last<T>(items: &[T]) -> Option<(&T, &T)> {
    Some((items.first()?, items.last()?))
}

// T appears only in the result, so the caller has to say which one.
fn zero<T: Default>() -> T {
    T::default()
}

fn main() {
    println!("numbers: {:?}", first_and_last(&[4, 8, 15, 16]));
    println!("words: {:?}", first_and_last(&["ant", "bee", "cat"]));
    println!("nothing: {:?}", first_and_last::<f64>(&[]));
    println!("zero::<i32>(): {}", zero::<i32>());
    let name: String = zero();
    println!("zero() as a String: {name:?}");
}
