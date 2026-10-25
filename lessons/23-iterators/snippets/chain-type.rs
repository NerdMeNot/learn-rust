use std::any::type_name_of_val;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    // A chain is a value: a struct wrapping a struct wrapping the slice iterator.
    let chain = nums.iter().filter(|&&n| n % 2 == 0).map(|n| n * n);

    // type_name is for debugging; its exact text isn't guaranteed between versions.
    println!("type: {}", type_name_of_val(&chain));

    // The closures capture nothing, so they take no space. What's left is the
    // slice iterator's two pointers: no heap allocation, no list of steps.
    println!("size: {} bytes", size_of_val(&chain));

    let result: Vec<i32> = chain.collect();
    println!("result: {result:?}");
}
