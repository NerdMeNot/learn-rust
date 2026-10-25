fn main() {
    let a = [1, 2, 3];
    let i = std::env::args().count() + 4; // 5, known only at run time
    println!("get: {:?}", a.get(i));
    println!("index: {}", a[i]);
}
