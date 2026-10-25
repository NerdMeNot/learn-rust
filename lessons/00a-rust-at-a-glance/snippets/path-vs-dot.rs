fn main() {
    let s = String::from("hello");
    println!("s.len(): {}", s.len());
    println!("String::len(&s): {}", String::len(&s));
    println!("i32::MAX: {}", i32::MAX);
    println!("std::cmp::max(3, 9): {}", std::cmp::max(3, 9));
}
