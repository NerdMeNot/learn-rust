fn first_of(Some(x): Option<i32>) -> i32 {
    x
}

fn main() {
    let items = [Some(1), None, Some(3)];
    for Some(n) in items {
        println!("{n}");
    }
    let double = |Some(n): Option<i32>| n * 2;
    println!("{} {}", first_of(Some(1)), double(Some(2)));
}
