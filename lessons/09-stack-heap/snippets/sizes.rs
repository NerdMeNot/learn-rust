use std::mem::size_of;

// We only measure this struct, so silence the "fields never read" warning.
#[allow(dead_code)]
struct Pair {
    x: i32,
    y: i32,
}

fn main() {
    println!("type               bytes");
    println!("i32                {:>5}", size_of::<i32>());
    println!("u64                {:>5}", size_of::<u64>());
    println!("f64                {:>5}", size_of::<f64>());
    println!("bool               {:>5}", size_of::<bool>());
    println!("char               {:>5}", size_of::<char>());
    println!("(i32, i32)         {:>5}", size_of::<(i32, i32)>());
    println!("[u8; 16]           {:>5}", size_of::<[u8; 16]>());
    println!("Pair               {:>5}", size_of::<Pair>());
    println!("&i32               {:>5}", size_of::<&i32>());
    println!("&str               {:>5}", size_of::<&str>());
    println!("String             {:>5}", size_of::<String>());
    println!("Vec<i32>           {:>5}", size_of::<Vec<i32>>());
    println!("Box<i32>           {:>5}", size_of::<Box<i32>>());
}
