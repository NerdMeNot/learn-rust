use std::any::type_name;

fn element_type<T>(_: &Vec<T>) -> &'static str {
    type_name::<T>()
}

fn main() {
    let words: Vec<String> = Vec::new();
    let numbers: Vec<i32> = vec![1000, 1000, 7];
    println!("{} and {}", element_type(&words), element_type(&numbers));
    let gap = numbers[1..].as_ptr().addr() - numbers.as_ptr().addr();
    println!("bytes between neighbours: {gap}");
    println!("{}", numbers[0] == numbers[1]);
}
