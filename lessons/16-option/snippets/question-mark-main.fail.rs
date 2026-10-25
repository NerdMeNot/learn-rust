fn first_even(numbers: [i32; 3]) -> Option<i32> {
    numbers.into_iter().find(|n| n % 2 == 0)
}

fn main() {
    let n = first_even([1, 2, 3])?;
    println!("first even: {n}");
}
