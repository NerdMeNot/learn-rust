fn first_even(numbers: [i32; 3]) -> Option<i32> {
    for n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    match first_even([1, 3, 5]) {
        Some(n) => println!("first even: {n}"),
    }
}
