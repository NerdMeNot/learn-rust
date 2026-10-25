fn main() {
    let a = vec![10, 20, 30];
    let b: Vec<i32> = Vec::new();
    let c = vec![0; 4];
    let d = Vec::from([1, 2, 3]);
    let e = Vec::<f64>::with_capacity(10);
    let f: Vec<u32> = (1..=5).collect();

    println!("vec![10, 20, 30]:  {a:?}");
    println!("Vec::new():        {b:?}");
    println!("vec![0; 4]:        {c:?}");
    println!("Vec::from([1, 2, 3]): {d:?}");
    println!(
        "with_capacity(10): len {}, capacity {}",
        e.len(),
        e.capacity()
    );
    println!("(1..=5).collect(): {f:?}");
}
