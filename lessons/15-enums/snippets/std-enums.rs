// std defines Option and Result as ordinary enums. Here is the same idea by hand.
// (`<T>` means "any type here"; generics come in a later lesson.)
#[derive(Debug)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    // the real Option: matched like any other enum
    match first_even(&[1, 3, 4, 5]) {
        Some(n) => println!("found {n}"),
        None => println!("no even number"),
    }

    // the real Result: Ok(value) or Err(error)
    let parsed: Result<i32, _> = "12x".parse::<i32>();
    match parsed {
        Ok(n) => println!("parsed {n}"),
        Err(e) => println!("could not parse: {e}"),
    }

    // our own copy works the same way
    let a: Maybe<i32> = Maybe::Just(7);
    let b: Maybe<i32> = Maybe::Nothing;
    println!("{a:?} and {b:?}");
}
