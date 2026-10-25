fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5].into_iter();
    let small: Vec<i32> = numbers.by_ref().take_while(|&n| n < 3).collect();
    let rest: Vec<i32> = numbers.collect();
    println!("{small:?} {rest:?}");
}
