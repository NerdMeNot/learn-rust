fn average(total: i32, count: i32) -> i32 {
    total / count
}

fn main() {
    println!("average of 10 over 2: {}", average(10, 2));
    println!("average of 10 over 0: {}", average(10, 0));
}
